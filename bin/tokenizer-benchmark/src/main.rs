
use std::time::Duration;

use criterion::{criterion_group, criterion_main};
use criterion::{Criterion, ParameterizedBenchmark};

use tokenizer_iterative1::{SingleInlineIteratorTokenizer, SingleIteratorTokenizer};
use tokenizer_nom3::{Nom3FoldTokenIterator, Nom3VecTokenIterator};
use tokenizer_nom4::{Nom4FoldTokenIterator, Nom4VecTokenIterator};
use tokenizer_nom5::{Nom5FoldTokenIterator, Nom5VecTokenIterator};
use tokenizer_regex::SingleFindRegexTokenizer;

const INPUT1: &'static str = " test1(test2 test3)test4 test5{test6 test7}test8 , . - +";

fn bench_tokenizers(c: &mut Criterion) {
    let inputs = vec![&INPUT1[..]];

    c.bench(
        "Tokenization",
        ParameterizedBenchmark::new(
            "SingleIterator",
            |b, i| {
                let mut output = Vec::with_capacity(15);
                b.iter(|| {
                    let iterator = SingleInlineIteratorTokenizer::from_slice(i.as_bytes());
                    output.extend(iterator);
                });
            },
            inputs,
        )
        .sample_size(50_000)
        .warm_up_time(Duration::from_secs(15))
        .with_function("SingleInlineIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = SingleIteratorTokenizer::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom5FoldIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom5FoldTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom5VecIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom5VecTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom4FoldIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom4FoldTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom4VecIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom4VecTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom3FoldIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom3FoldTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("Nom3VecIterator", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = Nom3VecTokenIterator::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        })
        .with_function("SingleFindRegexTokenizer", |b, i| {
            let mut output = Vec::with_capacity(15);
            b.iter(|| {
                let iterator = SingleFindRegexTokenizer::from_slice(i.as_bytes());
                output.extend(iterator);
            });
        }),
    );
}

criterion_group!(tokenizers, bench_tokenizers);
criterion_main!(tokenizers);
