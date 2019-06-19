
use std::marker::PhantomData;

use super::Nom3Tokenizer;

pub struct Nom3TokenIterator<'a, P>
where
    P: 'static + Nom3Tokenizer,
{
    remaining: &'a [u8],
    parser: PhantomData<P>,
}

impl<'a, P> Nom3TokenIterator<'a, P>
where
    P: 'static + Nom3Tokenizer,
{
    pub fn from_slice(slice: &'a [u8]) -> Nom3TokenIterator<'a, P> {
        Nom3TokenIterator {
            remaining: slice,
            parser: PhantomData,
        }
    }
}

impl<'a, P> Iterator for Nom3TokenIterator<'a, P>
where
    P: 'static + Nom3Tokenizer,
{
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.remaining.is_empty() {
            return None;
        }

        loop {
            match P::token(&self.remaining) {
                nom::IResult::Done(remaining, next) => {
                    self.remaining = remaining;
                    return Some(next);
                }
                nom::IResult::Incomplete(nom::Needed::Size(i)) if i > 0 => {
                    let next = self.remaining;
                    self.remaining = b"";
                    return Some(next);
                }
                _ => return None,
            }
        }
    }
}
