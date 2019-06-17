
mod iterator;

mod fold_tokenizer;
mod vec_tokenizer;

use nom::{is_not, named};

pub use iterator::Nom4TokenIterator;

pub type Nom4VecTokenIterator<'a> = Nom4TokenIterator<'a, vec_tokenizer::Nom4VecTokenizer>;
pub type Nom4FoldTokenIterator<'a> = Nom4TokenIterator<'a, fold_tokenizer::Nom4FoldTokenizer>;

named!(pub start_of_sequence<&[u8]>,
  is_not!(&b"()[],.-+*^/&|%!b\" \t\n\r"[..])
);

named!(pub start_of_sequence_without_b<&[u8]>,
  is_not!(&b"()[],.-+*^/&|%!\" \t\n\r"[..])
);

pub trait Nom4Tokenizer: Default {
    fn token(input: &[u8]) -> nom::IResult<&[u8], &[u8]>;
}
