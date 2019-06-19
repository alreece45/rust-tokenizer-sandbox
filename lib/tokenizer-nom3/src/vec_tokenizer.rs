
use super::Nom3Tokenizer;
use super::{start_of_sequence, start_of_sequence_without_b};

#[derive(Default)]
pub struct Nom3VecTokenizer {}

impl Nom3Tokenizer for Nom3VecTokenizer {
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

    use super::super::Nom3VecTokenIterator;

    test_tokenizer!(Nom3VecTokenIterator::from_slice);
}
