
#[macro_use]
mod macros;
pub mod tests;

#[macro_export]
macro_rules! test_tokenizer {
    ($create_tokenizer:expr) => {
        #[test]
        fn whitespace() {
            $crate::tests::whitespace($create_tokenizer);
        }

        #[test]
        fn parentheses() {
            $crate::tests::parentheses($create_tokenizer);
        }

        #[test]
        fn math_single_character() {
            $crate::tests::math_single($create_tokenizer);
        }

        #[test]
        fn math_double_character() {
            $crate::tests::math_double_char($create_tokenizer);
        }

        #[test]
        fn logical_ops() {
            $crate::tests::logical_ops($create_tokenizer);
        }

        #[test]
        fn other() {
            $crate::tests::unused($create_tokenizer);
        }

        #[test]
        fn strings() {
            $crate::tests::strings($create_tokenizer);
        }
    };
}