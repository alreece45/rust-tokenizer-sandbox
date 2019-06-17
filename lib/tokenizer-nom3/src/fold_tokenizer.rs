
use super::Nom3Tokenizer;
use super::{start_of_sequence, start_of_sequence_without_b};

#[doc(hidden)]
#[derive(Default)]
pub struct Nom3FoldTokenizer {}

impl Nom3Tokenizer for Nom3FoldTokenizer {
    named!(
        token<&[u8]>,
        ws!(alt!(
            call!(start_of_sequence)
                | recognize!(delimited!(
                    alt!(tag!("\"") | tag!("b\"")),
                    fold_many0!(
                        alt!(tag!("\\\"") | tag!("\\") | is_not!("\\\"")),
                        0u8,
                        |acc, item| -> u8 { acc }
                    ),
                    tag!("\"")
                ))
                | recognize!(preceded!(tag!("b"), call!(start_of_sequence_without_b)))
                | tag!(b"==") | tag!(b"!=")
                | tag!(b"&&") | tag!(b"||")
                | tag!(b"**") | tag!(b"..")
                | recognize!(one_of!("()[],.|&^!+-*/%"))
        ))
    );
}

#[cfg(test)]
mod tests {
    use tokenizer_common::test_tokenizer;

    use super::super::Nom3FoldTokenIterator;

    test_tokenizer!(Nom3FoldTokenIterator::from_slice);
}
