use lazy_static::lazy_static;
use regex::bytes::{Matches, Regex};

pub struct SingleFindRegexTokenizer<'a> {
    matches: Matches<'static, 'a>,
}

impl<'a> SingleFindRegexTokenizer<'a> {
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"(?x)(
                    (?:(:?b|r*)?"(\\"|[^"])+")|
                    (?:&&|\|\||==|!=|\*\*|\.\.)|
                    (?:[{}()\[\],.\-+*^/&|%!])|
                    (?:[^()\[\],.\-+*^/&|%!"\s]+)
                )"#
            )
            .unwrap();
        }
        Self {
            matches: RE.find_iter(bytes),
        }
    }
}

impl<'a> Iterator for SingleFindRegexTokenizer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.matches.next().map(|m| m.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::SingleFindRegexTokenizer;

    use tokenizer_tests::test_tokenizer;

    test_tokenizer!(SingleFindRegexTokenizer::from_slice);
}
