
use tokenizer_common::{is_not_whitespace, is_start_prefix, is_whitespace};

pub struct SingleIteratorTokenizer<'a> {
    remaining: &'a [u8],
}

impl<'a> SingleIteratorTokenizer<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        SingleIteratorTokenizer { remaining: slice }
    }

    pub fn parse_string_contents(&mut self, string_start: usize) -> Option<&'a [u8]> {
        let mut end = 0;
        for slice in (&self.remaining[string_start..]).split(|&c| c == b'"') {
            end += 1 + slice.len();
            if !slice.ends_with(b"\\") {
                break;
            }
        }

        if end == 0 {
            return None;
        }

        let token = &self.remaining[..(end + string_start)];
        self.remaining = &self.remaining[(end + string_start)..];
        Some(token)
    }
}

impl<'a> Iterator for SingleIteratorTokenizer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.remaining.iter();
        if let Some(start) = iter.position(|c| is_not_whitespace(*c)) {
            self.remaining = &self.remaining[start..];
        } else {
            return None;
        };

        let one = &self.remaining[0..1];
        if self.remaining.len() >= 2 {
            let together = &self.remaining[0..2];
            let two = &self.remaining[1..2];

            let double_token = match (one, two) {
                (b".", b".") => Some((together, 2)),
                (b".", _)     => Some((one, 1)),
                (b"=", b"=")  => Some((together, 2)),
                (b"=", _)     => Some((one, 1)),
                (b"!", b"=")  => Some((together, 2)),
                (b"!", _)     => Some((one, 1)),
                (b"&", b"&")  => Some((together, 2)),
                (b"&", _)     => Some((one, 1)),
                (b"|", b"|")  => Some((together, 2)),
                (b"|", _)     => Some((one, 1)),
                (b"*", b"*")  => Some((together, 2)),
                (b"*", _)     => Some((one, 1)),
                (b"b", b"\"") => return self.parse_string_contents(2),
                _ => None,
            };

            if let Some((token, end)) = double_token {
                self.remaining = &self.remaining[end..];
                return Some(token);
            }
        }

        match one {
            b"(" | b")" | b"[" | b"]" | b"," | b"." |
            b"|" | b"&" | b"-" | b"+" | b"%" | b"/" |
            b"&" | b"^" => {
                let token = &self.remaining[0..1];
                self.remaining = &self.remaining[1..];
                Some(token)
            }
            b"\"" => self.parse_string_contents(1),
            _ => {
                let token_end: usize = (&self.remaining[0..])
                    .iter()
                    .position(|&c| is_whitespace(c) || is_start_prefix(c))
                    .unwrap_or(self.remaining.len());

                let token = &self.remaining[..(token_end)];
                self.remaining = &self.remaining[token_end..];
                Some(token)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokenizer_common::test_tokenizer;

    use super::SingleIteratorTokenizer;

    test_tokenizer!(SingleIteratorTokenizer::from_slice);
}
