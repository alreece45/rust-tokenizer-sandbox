
#[inline]
pub fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' | b'\t' | b'\n' | b'\r' => true,
        _ => false,
    }
}

#[inline]
pub fn is_not_whitespace(c: u8) -> bool {
    !is_whitespace(c)
}

#[inline]
pub fn is_start_of_two(c: u8) -> bool {
    match c {
        b'=' | b'!' | b'&' | b'|' | b'*' | b'b' => true,
        _ => false,
    }
}

#[inline]
pub fn is_start_prefix(c: u8) -> bool {
    #[cfg_attr(rustfmt, rustfmt_skip)] 
    match c {
        b'.' | b'=' | b'!' | b'&' | b'|' |
        b'(' | b')' | b'[' | b']' | b',' |
        b'*' | b'-' | b'+' | b'/' | b'^' => true,
        _ => false,
    }
}

#[inline]
pub fn is_single_character_token(c: u8) -> bool {
    #[cfg_attr(rustfmt, rustfmt_skip)] 
    match c {
        b'(' | b')' | b'[' | b']' | b'{' | b'}' |
        b',' | b'.' | b'|' | b'&' | b'-' | b'+' |
        b'/' | b'^' => true,
        _ => false,
    }
}