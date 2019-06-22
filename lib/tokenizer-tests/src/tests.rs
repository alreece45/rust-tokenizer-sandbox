
pub fn whitespace<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    expect_tokens!(
        tokenizer_factory,
        b" \r\n\tthis\t\n\r is \r\n\t a \t\n\r test",

        b"this", b"is", b"a", b"test"
    );
}

pub fn parentheses<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    expect_tokens!(
        tokenizer_factory,
        b" (!test1000(test1001 test1002)test1003 () (( ))",
        b"(", b"!", b"test1000",
        b"(", b"test1001", b"test1002", b")",
        b"test1003",
        b"(", b")",
        b"(", b"(", b")", b")"
    );
}

pub fn math_single<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    expect_tokens!(
        tokenizer_factory,
        b"test2001 + test2002 - test2003 * test2004 / test2005 ++--*//",

        b"test2001", b"+", b"test2002", b"-",
        b"test2003", b"*", b"test2004", b"/",
        b"test2005", b"+", b"+", b"-", b"-", b"*", b"/", b"/"
    );
}

pub fn math_double_char<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    expect_tokens!(
        tokenizer_factory,
        b"test2100 ** test2101 ** test2102 ******",

        b"test2100", b"**", b"test2101", b"**", b"test2102",
        b"**", b"**", b"**"
    );
}

pub fn logical_ops<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    expect_tokens!(
        tokenizer_factory,
        b"test3001 && test3002 == test3003 || test3004 != test3005 !=||==&&",

        b"test3001", b"&&", b"test3002", b"==",
        b"test3003", b"||", b"test3004", b"!=",
        b"test3005", b"!=", b"||", b"==", b"&&"
    );
}

pub fn unused<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    expect_tokens!(
        tokenizer_factory,
        b"  [test9, test10, test11] , . - +",

        b"[", b"test9", b",", b"test10", b",", b"test11", b"]",
        b",", b".", b"-", b"+"
    );
}

pub fn strings<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
{
    expect_tokens!(
        tokenizer_factory, br#"
        "test 4001" + b"test 4002" + "test4003 ()\"*+-\\ test4004"
        + b"test4005 +-\" + test4006" + beginning_with_a_b"#,

        br#""test 4001""#, b"+", br#"b"test 4002""#, b"+",
        br#""test4003 ()\"*+-\\ test4004""#, b"+",
        br#"b"test4005 +-\" + test4006""#, b"+",
        b"beginning_with_a_b"
    );
}
