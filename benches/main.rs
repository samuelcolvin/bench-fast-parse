use criterion::{black_box, criterion_group, criterion_main, Criterion};

use int_bench::{fast_parse1, fast_parse2};

fn bench(c: &mut Criterion) {
    let pos_int0 = black_box("0");
    let pos_int1 = black_box("1");
    let pos_int123 = black_box("123");
    let pos_int_mid = black_box("12345678901234");
    let pos_int_long = black_box("1234567890123456");
    let neg_int0 = black_box("-0");
    let neg_int1 = black_box("-1");
    let neg_int123 = black_box("-123");
    let neg_int_mid = black_box("-12345678901234");
    let neg_int_long = black_box("-1234567890123456");

    c.bench_function("std", |b| b.iter(|| {
        black_box(str::parse::<i64>(pos_int0).unwrap());
        black_box(str::parse::<i64>(pos_int1).unwrap());
        black_box(str::parse::<i64>(pos_int123).unwrap());
        black_box(str::parse::<i64>(pos_int_mid).unwrap());
        black_box(str::parse::<i64>(pos_int_long).unwrap());
        black_box(str::parse::<i64>(neg_int0).unwrap());
        black_box(str::parse::<i64>(neg_int1).unwrap());
        black_box(str::parse::<i64>(neg_int123).unwrap());
        black_box(str::parse::<i64>(neg_int_mid).unwrap());
        black_box(str::parse::<i64>(neg_int_long).unwrap());
    }));

    c.bench_function("fast1", |b| b.iter(|| {
        black_box(fast_parse1(pos_int0).unwrap());
        black_box(fast_parse1(pos_int1).unwrap());
        black_box(fast_parse1(pos_int123).unwrap());
        black_box(fast_parse1(pos_int_mid).unwrap());
        black_box(fast_parse1(pos_int_long).unwrap());

        black_box(fast_parse1(neg_int0).unwrap());
        black_box(fast_parse1(neg_int1).unwrap());
        black_box(fast_parse1(neg_int123).unwrap());
        black_box(fast_parse1(neg_int_mid).unwrap());
        black_box(fast_parse1(neg_int_long).unwrap());
    }));

    c.bench_function("fast2", |b| b.iter(|| {
        black_box(fast_parse2(pos_int0).unwrap());
        black_box(fast_parse2(pos_int1).unwrap());
        black_box(fast_parse2(pos_int123).unwrap());
        black_box(fast_parse2(pos_int_mid).unwrap());
        black_box(fast_parse2(pos_int_long).unwrap());

        black_box(fast_parse2(neg_int0).unwrap());
        black_box(fast_parse2(neg_int1).unwrap());
        black_box(fast_parse2(neg_int123).unwrap());
        black_box(fast_parse2(neg_int_mid).unwrap());
        black_box(fast_parse2(neg_int_long).unwrap());
    }));
}

criterion_group!(arbitrary, bench);
criterion_main!(arbitrary);
