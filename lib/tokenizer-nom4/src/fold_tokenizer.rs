

use nom::{alt, call, delimited, fold_many0, is_not, named, one_of, preceded, recognize, tag, ws};

use super::Nom4Tokenizer;
use super::{start_of_sequence, start_of_sequence_without_b};

#[derive(Default)]
pub struct Nom4FoldTokenizer {}

impl Nom4Tokenizer for Nom4FoldTokenizer {
    named!(token<&[u8]>, ws!(
        alt!(
            call!(start_of_sequence)
            | recognize!(
                delimited!(
                    alt!(tag!("\"") | tag!("b\"")),
                    fold_many0!(
                        alt!(tag!("\\\"") | tag!("\\") | is_not!("\\\"")),
                        0u8, |acc, _| { acc }
                    ),
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
