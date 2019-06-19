
use std::marker::PhantomData;

use super::Nom5Tokenizer;

pub struct Nom5TokenIterator<'a, P: Nom5Tokenizer> {
    remaining: &'a [u8],
    parser: PhantomData<P>,
}

impl<'a, P: Nom5Tokenizer> Nom5TokenIterator<'a, P> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Nom5TokenIterator {
            remaining: slice,
            parser: PhantomData,
        }
    }
}

impl<'a, P: Nom5Tokenizer> Iterator for Nom5TokenIterator<'a, P> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        if self.remaining.is_empty() {
            return None;
        }

        loop {
            match P::token(&self.remaining) {
                Ok((remaining, next)) => {
                    self.remaining = remaining;
                    return Some(next);
                }
                Err(nom::Err::Incomplete(nom::Needed::Size(i))) if i > 0 => {
                    let next = self.remaining;
                    self.remaining = b"";
                    return Some(next);
                }
                _ => return None,
            }
        }
    }
}
