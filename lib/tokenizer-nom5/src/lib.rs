
mod iterator;

mod fold_tokenizer;
mod vec_tokenizer;

use nom::{is_not, named};

pub use iterator::Nom5TokenIterator;

pub type Nom5VecTokenIterator<'a> = Nom5TokenIterator<'a, vec_tokenizer::Nom5VecTokenizer>;
pub type Nom5FoldTokenIterator<'a> = Nom5TokenIterator<'a, fold_tokenizer::Nom5FoldTokenizer>;

pub trait Nom5Tokenizer: Default {
    fn token(input: &[u8]) -> nom::IResult<&[u8], &[u8]>;
}

named!(start_of_sequence<&[u8]>,
    is_not!(&b"()[],.-+*^/&|%!b\" \t\n\r"[..])
);

named!(start_of_sequence_without_b<&[u8]>,
  is_not!(&b"()[],.-+*^/&|%!\" \t\n\r"[..])
);
