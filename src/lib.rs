#![feature(test)]
#![feature(slice_ptr_get)]

extern crate test;
use core::arch::aarch64::*;

pub fn simd(bytes: &[u8]) -> Option<u64> {
    unsafe {
        let s: uint8x8_t = match bytes.len() {
            1 => return Some((bytes[0] & 0x0f) as u64),
            2 => return Some((bytes[0] & 0x0f) as u64 * 10 + (bytes[1] & 0x0f) as u64),
            // 3 => return (bytes[0] & 0x0f) as u64 * 100 + (bytes[1] & 0x0f) as u64 * 10 + (bytes[2] & 0x0f) as u64,
            3 => {
                let bytes8: [u8; 4] = [b'0', bytes[0], bytes[1], bytes[2]];
                return simd4(vld1_u8(bytes8.as_ptr()))
            }
            4 => return simd4(vld1_u8(bytes.as_ptr())),
            5 => {
                let bytes8: [u8; 8] = [b'0', b'0', b'0', bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]];
                vld1_u8(bytes8.as_ptr())
            }
            6 => {
                let bytes8: [u8; 8] = [b'0', b'0', bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]];
                vld1_u8(bytes8.as_ptr())
            }
            7 => {
                let bytes8: [u8; 8] = [
                    b'0', bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                ];
                vld1_u8(bytes8.as_ptr())
            }
            8 => vld1_u8(bytes.as_ptr()),
            _ => return None,
        };

        let zeros = vld1_u8([b'0'; 8].as_ptr());
        let mut ok: u8 = 0;
        let gte_0 = vcge_u8(s, zeros);
        vst1_u8(&mut ok, gte_0);
        if ok == 0 {
            return None
        }
        let lte_9 = vcle_u8(s, vld1_u8([b'9'; 8].as_ptr()));
        vst1_u8(&mut ok, lte_9);
        if ok == 0 {
            return None
        }

        let v = vsub_u8(s, zeros);

        let v = vmul_u8(v, vld1_u8([10_u8, 1_u8, 10_u8, 1_u8, 10_u8, 1_u8, 10_u8, 1_u8].as_ptr()));

        let v = vpaddl_u8(v);
        let v = vmul_u16(v, vld1_u16([100_u16, 1_u16, 100_u16, 1_u16].as_ptr()));

        let v = vpaddl_u16(v);
        let v = vmul_u32(v, vld1_u32([10_000_u32, 1_u32].as_ptr()));

        let v = vpaddl_u32(v);

        let mut result: u64 = 0;
        vst1_u64(&mut result, v);
        return Some(result);
    }
}

pub fn simd4(s: uint8x8_t) -> Option<u64> {
    unsafe {
        let v = vsub_u8(s, vld1_u8([b'0'; 4].as_ptr()));
        let v = vmul_u8(v, vld1_u8([10_u8, 1_u8, 10_u8, 1_u8].as_ptr()));

        let v = vpaddl_u8(v);
        let v = vmul_u16(v, vld1_u16([100_u16, 1_u16, 0_u16, 0_u16].as_ptr()));

        let v = vpaddl_u16(v);

        let mut result: u32 = 0;
        vst1_u32(&mut result, v);
        return Some(result as u64);
    }
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
            }
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
    #[test]
    fn simd() {
        assert_eq!(super::simd(b"1").unwrap(), 1);
        assert_eq!(super::simd(b"12").unwrap(), 12);
        assert_eq!(super::simd(b"0012").unwrap(), 12);
        assert_eq!(super::simd(b"123").unwrap(), 123);
        assert_eq!(super::simd(b"1234").unwrap(), 1234);
        assert_eq!(super::simd(b"12345").unwrap(), 12345);
        assert_eq!(super::simd(b"123456").unwrap(), 123456);
        assert_eq!(super::simd(b"1234567").unwrap(), 1234567);
        assert_eq!(super::simd(b"12345678").unwrap(), 12345678);
        assert_eq!(super::simd(b"99999999").unwrap(), 99999999);
        assert_eq!(super::simd(b"!9999999"), None);
        assert_eq!(super::simd(b"a9999999"), None);
    }

    #[test]
    fn test_fast_parse1() {
        assert_eq!(super::fast_parse1("0").unwrap(), 0);
        assert_eq!(super::fast_parse1("1").unwrap(), 1);
        assert_eq!(super::fast_parse1("-1").unwrap(), -1);
        assert_eq!(super::fast_parse1("123").unwrap(), 123);
        assert_eq!(super::fast_parse1("-123").unwrap(), -123);
        assert_eq!(super::fast_parse1("1585201087123789").unwrap(), 1585201087123789);
        assert_eq!(super::fast_parse1("1234567890123456").unwrap(), 1234567890123456);
        assert_eq!(super::fast_parse1("1111111111111111").unwrap(), 1111111111111111);
        assert_eq!(
            super::fast_parse1(&(i64::MAX as u64 - 1).to_string()).unwrap(),
            9223372036854775806
        );
    }

    #[test]
    fn test_fast_parse2() {
        assert_eq!(super::fast_parse2("0").unwrap(), 0);
        assert_eq!(super::fast_parse2("1").unwrap(), 1);
        assert_eq!(super::fast_parse2("-1").unwrap(), -1);
        assert_eq!(super::fast_parse2("123").unwrap(), 123);
        assert_eq!(super::fast_parse2("-123").unwrap(), -123);
        assert_eq!(super::fast_parse2("1585201087123789").unwrap(), 1585201087123789);
        assert_eq!(super::fast_parse2("1234567890123456").unwrap(), 1234567890123456);
        assert_eq!(super::fast_parse2("1111111111111111").unwrap(), 1111111111111111);
        assert_eq!(
            super::fast_parse2(&(i64::MAX as u64 - 1).to_string()).unwrap(),
            9223372036854775806
        );
    }
}
