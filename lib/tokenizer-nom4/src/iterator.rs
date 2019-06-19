
use std::marker::PhantomData;

use super::Nom4Tokenizer;

pub struct Nom4TokenIterator<'a, P>
    where P: Nom4Tokenizer
{
    remaining: &'a [u8],
    parser: PhantomData<P>,
}

impl<'a, P> Nom4TokenIterator<'a, P>
    where P: Nom4Tokenizer
{
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Nom4TokenIterator {
            remaining: slice,
            parser: PhantomData
        }
    }
}

impl<'a, P> Iterator for Nom4TokenIterator<'a, P>
    where P: Nom4Tokenizer
{
    type Item = & 'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.remaining.is_empty() {
            return None;
        }

        loop {
            match P::token(&self.remaining) {
                Ok((remaining, next)) => {
                    self.remaining = remaining;
                    return Some(next)
                },
                Err(nom::Err::Incomplete(nom::Needed::Size(i))) if i > 0 => {
                    let next = self.remaining;
                    self.remaining = b"";
                    return Some(next);
                }
                _ => {
                    return None
                }
            }
        }
    }
}
