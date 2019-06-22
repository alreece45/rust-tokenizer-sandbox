
use nom::{alt, call, delimited, fold_many0, is_not, named, one_of, preceded, recognize, tag, ws};

use super::Nom5Tokenizer;
use super::{start_of_sequence, start_of_sequence_without_b};

#[derive(Default)]
pub struct Nom5FoldTokenizer {}

named!(string_contents<&[u8]>,
    alt!(tag!("\\\"") | tag!("\\") | is_not!("\\\""))
);

impl Nom5Tokenizer for Nom5FoldTokenizer {
    named!(token<&[u8]>, ws!(
        alt!(
            call!(start_of_sequence)
            | recognize!(
                delimited!(
                    alt!(tag!("\"") | tag!("b\"")),
                    fold_many0!(
                        call!(string_contents),
                        0u8, |acc, _| -> u8 { acc }
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
    use tokenizer_tests::test_tokenizer;

    use super::super::Nom5VecTokenIterator;

    test_tokenizer!(Nom5VecTokenIterator::from_slice);
}
