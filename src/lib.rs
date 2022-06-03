#![feature(test)]

extern crate test;

fn main() {
    println!("18446744073709551615: {}", fast_parse("18446744073709551615").unwrap());
}

pub fn fast_parse1(s: &str) -> Option<i64> {
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

pub fn fast_parse2(s: &str) -> Option<i64> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return None;
    }
    let mut result: u64 = 0;
    let pos: bool;
    unsafe {
        pos = match bytes.get_unchecked(0) {
            b'-' => false,
            b'+' => true,
            c if (b'0'..=b'9').contains(c) => {
                result = (c & 0x0f) as u64;
                true
            },
            _ => return None,
        };

        let mut i = 1;
        loop {
            if i == len {
                return if pos {
                    Some(result as i64)
                } else {
                    Some(-1 * result as i64)
                };
            }
            let digit = bytes.get_unchecked(i);
            if (b'0'..=b'9').contains(digit) {
                result = result * 10 + (digit & 0x0f) as u64;
                i += 1;
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{fast_parse1, fast_parse2};

    #[test]
    fn test_fast_parse1() {
        assert_eq!(fast_parse1("0").unwrap(), 0);
        assert_eq!(fast_parse1("1").unwrap(), 1);
        assert_eq!(fast_parse1("-1").unwrap(), -1);
        assert_eq!(fast_parse1("123").unwrap(), 123);
        assert_eq!(fast_parse1("-123").unwrap(), -123);
        assert_eq!(fast_parse1("1585201087123789").unwrap(), 1585201087123789);
        assert_eq!(fast_parse1("1234567890123456").unwrap(), 1234567890123456);
        assert_eq!(fast_parse1("1111111111111111").unwrap(), 1111111111111111);
        assert_eq!(fast_parse1(&(i64::MAX as u64 - 1).to_string()).unwrap(), 9223372036854775806);
    }

    #[test]
    fn test_fast_parse2() {
        assert_eq!(fast_parse2("0").unwrap(), 0);
        assert_eq!(fast_parse2("1").unwrap(), 1);
        assert_eq!(fast_parse2("-1").unwrap(), -1);
        assert_eq!(fast_parse2("123").unwrap(), 123);
        assert_eq!(fast_parse2("-123").unwrap(), -123);
        assert_eq!(fast_parse2("1585201087123789").unwrap(), 1585201087123789);
        assert_eq!(fast_parse2("1234567890123456").unwrap(), 1234567890123456);
        assert_eq!(fast_parse2("1111111111111111").unwrap(), 1111111111111111);
        assert_eq!(fast_parse2(&(i64::MAX as u64 - 1).to_string()).unwrap(), 9223372036854775806);
    }
}
