use criterion::{black_box, criterion_group, criterion_main, Criterion};

use int_bench::{fast_parse1, simd};

fn bench(c: &mut Criterion) {
    let pos_int0 = black_box("1");
    let pos_int1 = black_box("0012");
    let pos_int123 = black_box("1234");
    let pos_int_mid = black_box("12345678");
    let pos_int_long = black_box("99999999");
    // let pos_int_mid = black_box("12345678901234");
    // let pos_int_long = black_box("1234567890123456");
    // let neg_int0 = black_box("-0");
    // let neg_int1 = black_box("-1");
    // let neg_int123 = black_box("-123");
    // let neg_int_mid = black_box("-12345678901234");
    // let neg_int_long = black_box("-1234567890123456");

    c.bench_function("std", |b| {
        b.iter(|| {
            black_box(str::parse::<i64>(pos_int0).unwrap());
            black_box(str::parse::<i64>(pos_int1).unwrap());
            black_box(str::parse::<i64>(pos_int123).unwrap());
            black_box(str::parse::<i64>(pos_int_mid).unwrap());
            black_box(str::parse::<i64>(pos_int_long).unwrap());

            // black_box(str::parse::<i64>(neg_int0).unwrap());
            // black_box(str::parse::<i64>(neg_int1).unwrap());
            // black_box(str::parse::<i64>(neg_int123).unwrap());
            // black_box(str::parse::<i64>(neg_int_mid).unwrap());

            // black_box(str::parse::<i64>(neg_int_long).unwrap());
        })
    });

    c.bench_function("fast1", |b| {
        b.iter(|| {
            black_box(fast_parse1(pos_int0).unwrap());
            black_box(fast_parse1(pos_int1).unwrap());
            black_box(fast_parse1(pos_int123).unwrap());
            black_box(fast_parse1(pos_int_mid).unwrap());
            black_box(fast_parse1(pos_int_long).unwrap());

            // black_box(fast_parse1(neg_int0).unwrap());
            // black_box(fast_parse1(neg_int1).unwrap());
            // black_box(fast_parse1(neg_int123).unwrap());
            // black_box(fast_parse1(neg_int_mid).unwrap());
            // black_box(fast_parse1(neg_int_long).unwrap());
        })
    });

    c.bench_function("simd", |b| {
        b.iter(|| {
            black_box(simd(pos_int0.as_bytes()).unwrap());
            black_box(simd(pos_int1.as_bytes()).unwrap());
            black_box(simd(pos_int123.as_bytes()).unwrap());
            black_box(simd(pos_int_mid.as_bytes()).unwrap());
            black_box(simd(pos_int_long.as_bytes()).unwrap());
        })
    });
}

criterion_group!(arbitrary, bench);
criterion_main!(arbitrary);
