
#[macro_use]
extern crate nom;

mod iterator;

mod fold_tokenizer;
mod vec_tokenizer;

use iterator::Nom3TokenIterator;

pub type Nom3VecTokenIterator<'a> = Nom3TokenIterator<'a, vec_tokenizer::Nom3VecTokenizer>;
pub type Nom3FoldTokenIterator<'a> = Nom3TokenIterator<'a, fold_tokenizer::Nom3FoldTokenizer>;

named!(
    start_of_sequence<&[u8]>,
    is_not!(&b"()[],.-+*^/&|%!b\" \t\n\r"[..])
);

named!(
    start_of_sequence_without_b<&[u8]>,
    is_not!(&b"()[],.-+*^/&|%!\" \t\n\r"[..])
);

pub trait Nom3Tokenizer: Default {
    fn token(input: &[u8]) -> nom::IResult<&[u8], &[u8]>;
}
