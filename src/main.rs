#![feature(test)]

extern crate test;

use core::str::Bytes;

fn main() {
    println!(
        "18446744073709551615: {}",
        fast_parse_int("18446744073709551615").unwrap()
    );
}

/// This is around 2x faster than using `str::parse::<i64>()`
pub fn fast_parse_int(s: &str) -> Option<i64> {
    _fast_parse_int(s.bytes())
}

pub fn b_fast_parse_int(s: &[u8]) -> Option<i64> {
    _fast_parse_int(s.iter().map(|&c| c))
}

pub fn _fast_parse_int<I: Iterator>(mut bytes: I) -> Option<i64>
where
    I: Iterator<Item = u8>,
{
    let mut result: i64 = 0;
    let neg = match bytes.next() {
        Some(b'-') => true,
        Some(c) if (b'0'..=b'9').contains(&c) => {
            result = (c & 0x0f) as i64;
            false
        }
        _ => return None,
    };

    for digit in bytes {
        match digit {
            b'0'..=b'9' => {
                result = result.checked_mul(10)?;
                result = result.checked_add((digit & 0x0f) as i64)?
            }
            _ => return None,
        }
    }
    if neg {
        result = -result;
    }
    Some(result)
}

pub fn fast_parse_float(s: &str) -> Option<f64> {
    let mut bytes = s.bytes();
    let mut result_whole: i64 = 0;
    let neg = match bytes.next() {
        Some(b'-') => true,
        Some(c) if (b'0'..=b'9').contains(&c) => {
            result_whole = (c & 0x0f) as i64;
            false
        }
        _ => return None,
    };

    let mut found_dot = false;

    loop {
        let digit = match bytes.next() {
            Some(c) => c,
            None => break,
        };
        match digit {
            b'0'..=b'9' => {
                result_whole = result_whole.checked_mul(10)?;
                result_whole = result_whole.checked_add((digit & 0x0f) as i64)?;
            }
            b'.' => {
                found_dot = true;
                break;
            }
            _ => return None,
        }
    }

    let mut result = result_whole as f64;
    if found_dot {
        let mut div = 10_f64;
        for digit in bytes {
            match digit {
                b'0'..=b'9' => {
                    result += (digit & 0x0f) as f64 / div;
                    div *= 10_f64;
                }
                _ => return None,
            }
        }
    }
    if neg {
        result = -result;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::{b_fast_parse_int, fast_parse_float, fast_parse_int};
    use test::{black_box, Bencher};

    #[test]
    fn test_fast_parse_int() {
        assert_eq!(fast_parse_int("0").unwrap(), 0);
        assert_eq!(fast_parse_int("1").unwrap(), 1);
        assert_eq!(fast_parse_int("-1").unwrap(), -1);
        assert_eq!(fast_parse_int("123").unwrap(), 123);
        assert_eq!(fast_parse_int("-123").unwrap(), -123);
        assert_eq!(fast_parse_int("1577836800").unwrap(), 1577836800);
        assert_eq!(
            fast_parse_int("1585201087123789").unwrap(),
            1585201087123789
        );
        assert_eq!(
            fast_parse_int("1234567890123456").unwrap(),
            1234567890123456
        );
        assert_eq!(
            fast_parse_int("1111111111111111").unwrap(),
            1111111111111111
        );
        assert_eq!(
            fast_parse_int(&(i64::MAX - 1).to_string()).unwrap(),
            i64::MAX - 1
        );
    }

    fn floats_equal(a: f64, b: f64) -> bool {
        let mut threshold = 0.0000000001;
        if a.abs() > 1.0 {
            threshold *= a.abs();
        }
        if (a - b).abs() < threshold {
            true
        } else {
            println!("{} != {}", a, b);
            false
        }
    }

    #[test]
    fn test_fast_parse_float() {
        assert!(floats_equal(fast_parse_float("0").unwrap(), 0.0_f64));
        assert!(floats_equal(fast_parse_float("1.0").unwrap(), 1.0_f64));
        assert!(floats_equal(fast_parse_float("1.5").unwrap(), 1.5_f64));
        assert!(floats_equal(fast_parse_float("1.5").unwrap(), 1.5_f64));
        assert!(floats_equal(fast_parse_float("-1.99").unwrap(), -1.99_f64));
        assert!(floats_equal(
            fast_parse_float("-123.000123").unwrap(),
            -123.000123_f64
        ));
        assert!(floats_equal(
            fast_parse_float("158520108.7123789").unwrap(),
            158520108.7123789_f64
        ));
    }

    #[bench]
    fn int_bench_fast_parse(b: &mut Bencher) {
        let int0 = black_box("0");
        let int1 = black_box("1");
        let int123 = black_box("123");
        let int_mid = black_box("12345678901234");
        let int_long = black_box("1234567800000000");
        b.iter(|| {
            black_box(fast_parse_int(int0).unwrap());
            black_box(fast_parse_int(int1).unwrap());
            black_box(fast_parse_int(int123).unwrap());
            black_box(fast_parse_int(int_mid).unwrap());
            black_box(fast_parse_int(int_long).unwrap());
        });
    }

    #[bench]
    fn int_bench_std_parse(b: &mut Bencher) {
        let int0 = black_box("0");
        let int1 = black_box("1");
        let int123 = black_box("123");
        let int_mid = black_box("12345678901234");
        let int_long = black_box("1234567800000000");
        b.iter(|| {
            black_box(str::parse::<i64>(int0).unwrap());
            black_box(str::parse::<i64>(int1).unwrap());
            black_box(str::parse::<i64>(int123).unwrap());
            black_box(str::parse::<i64>(int_mid).unwrap());
            black_box(str::parse::<i64>(int_long).unwrap());
        });
    }

    #[bench]
    fn float_bench_fast_parse(b: &mut Bencher) {
        let float0 = black_box("0");
        let float1 = black_box("1");
        let float123 = black_box("123");
        let float_mid = black_box("12345678901234");
        let float_long = black_box("1234567800000000");
        b.iter(|| {
            black_box(fast_parse_float(float0).unwrap());
            black_box(fast_parse_float(float1).unwrap());
            black_box(fast_parse_float(float123).unwrap());
            black_box(fast_parse_float(float_mid).unwrap());
            black_box(fast_parse_float(float_long).unwrap());
        });
    }

    #[bench]
    fn float_bench_std_parse(b: &mut Bencher) {
        let float0 = black_box("0");
        let float1 = black_box("1.123");
        let float123 = black_box("123.321");
        let float_mid = black_box("12345678.901234");
        let float_long = black_box("1234567800000000.0");
        b.iter(|| {
            black_box(str::parse::<f64>(float0).unwrap());
            black_box(str::parse::<f64>(float1).unwrap());
            black_box(str::parse::<f64>(float123).unwrap());
            black_box(str::parse::<f64>(float_mid).unwrap());
            black_box(str::parse::<f64>(float_long).unwrap());
        });
    }
}
