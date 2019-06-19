

use nom::{alt, call, delimited, is_not, many0, named, one_of, preceded, recognize, tag, ws};

use super::Nom4Tokenizer;
use super::{start_of_sequence, start_of_sequence_without_b};

#[derive(Default)]
pub struct Nom4VecTokenizer {}

impl Nom4Tokenizer for Nom4VecTokenizer {
    named!(token<&[u8]>, ws!(
        alt!(
            call!(start_of_sequence)
            | recognize!(
                delimited!(
                    alt!(tag!("\"") | tag!("b\"")),
                    many0!(alt!(tag!("\\\"") | tag!("\\") | is_not!("\\\""))),
                    tag!("\"")
                )
            )
            | recognize!( preceded!(tag!("b"), call!(start_of_sequence_without_b)) )
            | tag!(b"==") | tag!(b"!=") | tag!(b"&&")
            | tag!(b"||") | tag!(b"**") | tag!(b"..")
            | recognize!(one_of!("()[],.|&^!+-*/%"))
        )
    ));
}

#[cfg(test)]
mod tests {
    use tokenizer_common::test_tokenizer;

    use super::super::Nom4VecTokenIterator;

    test_tokenizer!(Nom4VecTokenIterator::from_slice);
}
