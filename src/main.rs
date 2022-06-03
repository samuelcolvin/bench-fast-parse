#![feature(test)]

extern crate test;

fn main() {
    println!("18446744073709551615: {}", fast_parse("18446744073709551615").unwrap());
}

/// This is around 40x faster than using `str::parse::<i64>()`
pub fn fast_parse(s: &str) -> Option<i64> {
    let mut bytes = s.bytes();
    let mut result: u64 = 0;
    let sign: i64 = match bytes.next() {
        Some(b'-') => -1,
        Some(c) if (b'0'..=b'9').contains(&c) => {
            result = (c & 0x0f) as u64;
            1
        }
        _ => return None,
    };

    for digit in bytes {
        match digit {
            b'0'..=b'9' => {
                result *= 10;
                result += (digit & 0x0f) as u64;
                if result >= i64::MAX as u64 {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(sign * (result as i64))
}

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use crate::{fast_parse};

    #[test]
    fn test_fast_parse() {
        assert_eq!(fast_parse("0").unwrap(), 0);
        assert_eq!(fast_parse("1").unwrap(), 1);
        assert_eq!(fast_parse("-1").unwrap(), -1);
        assert_eq!(fast_parse("123").unwrap(), 123);
        assert_eq!(fast_parse("-123").unwrap(), -123);
        assert_eq!(fast_parse("1585201087123789").unwrap(), 1585201087123789);
        assert_eq!(fast_parse("1234567890123456").unwrap(), 1234567890123456);
        assert_eq!(fast_parse("1111111111111111").unwrap(), 1111111111111111);
        assert_eq!(fast_parse(&(i64::MAX as u64 - 1).to_string()).unwrap(), 9223372036854775806);
    }

    #[bench]
    fn bench_fast_parse(b: &mut Bencher) {
        let int0 = black_box("0");
        let int1 = black_box("1");
        let int123 = black_box("123");
        let int_mid = black_box("12345678901234");
        let int_long = black_box("1234567890123456");
        b.iter(|| {
            black_box(fast_parse(int0).unwrap());
            black_box(fast_parse(int1).unwrap());
            black_box(fast_parse(int123).unwrap());
            black_box(fast_parse(int_mid).unwrap());
            black_box(fast_parse(int_long).unwrap());
        });
    }

    #[bench]
    fn bench_std_parse(b: &mut Bencher) {
        let int0 = black_box("0");
        let int1 = black_box("1");
        let int123 = black_box("123");
        let int_mid = black_box("12345678901234");
        let int_long = black_box("1234567890123456");
        b.iter(|| {
            black_box(str::parse::<i64>(int0).unwrap());
            black_box(str::parse::<i64>(int1).unwrap());
            black_box(str::parse::<i64>(int123).unwrap());
            black_box(str::parse::<i64>(int_mid).unwrap());
            black_box(str::parse::<i64>(int_long).unwrap());
        });
    }
}
