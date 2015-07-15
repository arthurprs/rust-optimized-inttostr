use std::fmt::{self, Debug, Display};
use std::mem;
use std::num::Wrapping;
use std::str;
use std::ptr;

trait AsUInt {
	fn as_u64(self) -> u64;
	fn as_u32(self) -> u32;
}

macro_rules! impl_AsUInt {
	($($t:ident),*) => ($(impl AsUInt for $t {
		#[inline(always)]
        fn as_u64(self) -> u64 { self as u64 }
		#[inline(always)]
        fn as_u32(self) -> u32 { self as u32 }
    })*)
}

impl_AsUInt! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Wrapper<T: Debug + Copy + PartialEq + PartialOrd>(pub T);

const DEC_DIGITS_LUT: [u8; 200] = [
    '0' as u8,'0' as u8,'0' as u8,'1' as u8,'0' as u8,'2' as u8,'0' as u8,'3' as u8,'0' as u8,'4' as u8,'0' as u8,'5' as u8,'0' as u8,'6' as u8,'0' as u8,'7' as u8,'0' as u8,'8' as u8,'0' as u8,'9' as u8,
    '1' as u8,'0' as u8,'1' as u8,'1' as u8,'1' as u8,'2' as u8,'1' as u8,'3' as u8,'1' as u8,'4' as u8,'1' as u8,'5' as u8,'1' as u8,'6' as u8,'1' as u8,'7' as u8,'1' as u8,'8' as u8,'1' as u8,'9' as u8,
    '2' as u8,'0' as u8,'2' as u8,'1' as u8,'2' as u8,'2' as u8,'2' as u8,'3' as u8,'2' as u8,'4' as u8,'2' as u8,'5' as u8,'2' as u8,'6' as u8,'2' as u8,'7' as u8,'2' as u8,'8' as u8,'2' as u8,'9' as u8,
    '3' as u8,'0' as u8,'3' as u8,'1' as u8,'3' as u8,'2' as u8,'3' as u8,'3' as u8,'3' as u8,'4' as u8,'3' as u8,'5' as u8,'3' as u8,'6' as u8,'3' as u8,'7' as u8,'3' as u8,'8' as u8,'3' as u8,'9' as u8,
    '4' as u8,'0' as u8,'4' as u8,'1' as u8,'4' as u8,'2' as u8,'4' as u8,'3' as u8,'4' as u8,'4' as u8,'4' as u8,'5' as u8,'4' as u8,'6' as u8,'4' as u8,'7' as u8,'4' as u8,'8' as u8,'4' as u8,'9' as u8,
    '5' as u8,'0' as u8,'5' as u8,'1' as u8,'5' as u8,'2' as u8,'5' as u8,'3' as u8,'5' as u8,'4' as u8,'5' as u8,'5' as u8,'5' as u8,'6' as u8,'5' as u8,'7' as u8,'5' as u8,'8' as u8,'5' as u8,'9' as u8,
    '6' as u8,'0' as u8,'6' as u8,'1' as u8,'6' as u8,'2' as u8,'6' as u8,'3' as u8,'6' as u8,'4' as u8,'6' as u8,'5' as u8,'6' as u8,'6' as u8,'6' as u8,'7' as u8,'6' as u8,'8' as u8,'6' as u8,'9' as u8,
    '7' as u8,'0' as u8,'7' as u8,'1' as u8,'7' as u8,'2' as u8,'7' as u8,'3' as u8,'7' as u8,'4' as u8,'7' as u8,'5' as u8,'7' as u8,'6' as u8,'7' as u8,'7' as u8,'7' as u8,'8' as u8,'7' as u8,'9' as u8,
    '8' as u8,'0' as u8,'8' as u8,'1' as u8,'8' as u8,'2' as u8,'8' as u8,'3' as u8,'8' as u8,'4' as u8,'8' as u8,'5' as u8,'8' as u8,'6' as u8,'8' as u8,'7' as u8,'8' as u8,'8' as u8,'8' as u8,'9' as u8,
    '9' as u8,'0' as u8,'9' as u8,'1' as u8,'9' as u8,'2' as u8,'9' as u8,'3' as u8,'9' as u8,'4' as u8,'9' as u8,'5' as u8,'9' as u8,'6' as u8,'9' as u8,'7' as u8,'9' as u8,'8' as u8,'9' as u8,'9' as u8
];

macro_rules! impl_Display {
	($($t:ident),*: $conv_fn:ident) => ($(
	impl Display for Wrapper<$t> {
		#[allow(unused_comparisons)]
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			let Wrapper(n) = *self;
			let is_positive = n >= 0;
			let mut n = if ! is_positive {
				let Wrapping(n) = Wrapping(!n.$conv_fn()) + Wrapping(1);
				n
			} else {
				n.$conv_fn()
			};
			let mut buf: [u8; 20] = unsafe { mem::uninitialized() };
			let mut curr = buf.len() as isize;
			let buf_ptr = &mut buf[0] as *mut u8;
			let lut_ptr = &DEC_DIGITS_LUT as *const u8;

			unsafe {
				// eagerly decode 4 characters at a time
				if <$t>::max_value() as u64 >= 10000 {
					while n >= 10000 {
						let rem = (n % 10000) as isize;
						n /= 10000;

						let d1 = (rem / 100) << 1;
				        let d2 = (rem % 100) << 1;
						curr -= 4;
						ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
						ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
					}
				}

				// if we reach here numbers are <= 9999, so at most 4 chars long
				let mut n = n as isize; // possibly reduce 64bit math

				// decode 2 more chars, if > 2 chars
				if n >= 100 {
					let d1 = (n % 100) << 1;
					n /= 100;
					curr -= 2;
					ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
				}

				// decode last 1 or 2 chars
				if n < 10 {
					curr -= 1;
					*buf_ptr.offset(curr) = (n as u8) + 48;
				} else {
					let d1 = n << 1;
					curr -= 2;
					ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
				}
			}

	        f.pad_integral(is_positive, "", unsafe { str::from_utf8_unchecked(&buf[curr as usize..]) })
		}
	})*);
}

impl_Display!(i8, u8, i16, u16, i32, u32: as_u32);
impl_Display!(i64, u64: as_u64);
#[cfg(target_pointer_width = "32")]
impl_Display!(isize, usize: as_u32);
#[cfg(target_pointer_width = "64")]
impl_Display!(isize, usize: as_u64);

/// An error which can be returned when parsing an integer.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseIntError { kind: IntErrorKind }

#[derive(Debug, Clone, PartialEq)]
enum IntErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}


#[doc(hidden)]
trait FromStrRadixHelper: Copy + PartialOrd {
    fn min_value() -> Self;
    fn from_u32(u: u32) -> Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_sub(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}

// macro_rules! doit {
//     ($($t:ty)*) => ($(impl FromStrRadixHelper for Wrapper<$t> {
//         fn min_value() -> Self { Wrapper(<$t>::min_value()) }
//         fn from_u32(u: u32) -> Self { Wrapper(u as $t) }
//         fn checked_mul(&self, other: u32) -> Option<Self> {
//             unsafe { mem::transmute(self.0.checked_mul(other as $t)) }
//         }
//         fn checked_sub(&self, other: u32) -> Option<Self> {
//             unsafe { mem::transmute(self.0.checked_sub(other as $t)) }
//         }
//         fn checked_add(&self, other: u32) -> Option<Self> {
//             unsafe { mem::transmute(self.0.checked_add(other as $t)) }
//         }
//     })*)
// }
// doit! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

macro_rules! doit {
    ($($t:ty)*) => ($(impl FromStrRadixHelper for $t {
        fn min_value() -> Self { Self::min_value() }
        fn from_u32(u: u32) -> Self { u as Self }
        fn checked_mul(&self, other: u32) -> Option<Self> {
            Self::checked_mul(*self, other as Self)
        }
        fn checked_sub(&self, other: u32) -> Option<Self> {
            Self::checked_sub(*self, other as Self)
        }
        fn checked_add(&self, other: u32) -> Option<Self> {
            Self::checked_add(*self, other as Self)
        }
    })*)
}
doit! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }


fn from_str_radix<T: FromStrRadixHelper>(src: &str, radix: u32)
                                         -> Result<T, ParseIntError> {
 	const MINUS: u8 = '-' as u8;
    use self::IntErrorKind::*;
    use self::ParseIntError as PIE;

    assert!(radix >= 2 && radix <= 36,
           "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
           radix);

    if src.is_empty() {
    	return Err(PIE { kind: Empty });
    }

    let is_signed_ty = T::from_u32(0) > T::min_value();
    // all valid digits are ascii
    let src = src.as_bytes();

    match (src[0], &src[1..])  {
        (MINUS, src) if src.is_empty() => Err(PIE { kind: Empty }),
        (MINUS, src) if is_signed_ty => {
            // The number is negative
            let mut result = T::from_u32(0);
            for &c in src {
                let x = match (c as char).to_digit(radix) {
                    Some(x) => x,
                    None => return Err(PIE { kind: InvalidDigit }),
                };
                result = match result.checked_mul(radix) {
                    Some(result) => result,
                    None => return Err(PIE { kind: Underflow }),
                };
                result = match result.checked_sub(x) {
                    Some(result) => result,
                    None => return Err(PIE { kind: Underflow }),
                };
            }
            Ok(result)
        },
        (c, src) => {
            // The number is signed
            let mut result = match (c as char).to_digit(radix) {
                Some(x) => T::from_u32(x),
                None => return Err(PIE { kind: InvalidDigit }),
            };
            for &c in src {
                let x = match (c as char).to_digit(radix) {
                    Some(x) => x,
                    None => return Err(PIE { kind: InvalidDigit }),
                };
                result = match result.checked_mul(radix) {
                    Some(result) => result,
                    None => return Err(PIE { kind: Overflow }),
                };
                result = match result.checked_add(x) {
                    Some(result) => result,
                    None => return Err(PIE { kind: Overflow }),
                };
            }
            Ok(result)
        }
    }
}

impl<T> str::FromStr for Wrapper<T>
		where T: FromStrRadixHelper + Debug + Copy + PartialEq + PartialOrd {
	type Err = ParseIntError;
	fn from_str(src: &str) -> Result<Self, ParseIntError> {
        from_str_radix(src, 10).map(|i| Wrapper(i))
    }
}

#[cfg(test)]
mod from_str_tests {
	use super::{Wrapper, IntErrorKind, ParseIntError};
	use std::str::FromStr;
	use rand::{self, Rng};

	#[test]
	fn test_from_str() {
		assert_eq!(FromStr::from_str("11"), Ok(Wrapper(11)));
		assert_eq!(FromStr::from_str("-11"), Ok(Wrapper(-11)));
	}

	#[test]
	fn test_overflow() {
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("-129");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Underflow }));
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("128");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Overflow }));
	}

	#[test]
	fn test_invalid() {
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("--129");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("¼²¼");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
	}

	#[test]
	fn test_empty() {
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("-");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Empty }));
		let conv: Result<Wrapper<u8>, _> = FromStr::from_str("-");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Empty }));
		let conv: Result<Wrapper<i8>, _> = FromStr::from_str("");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Empty }));
		let conv: Result<Wrapper<u8>, _> = FromStr::from_str("");
		assert_eq!(conv, Err(ParseIntError{ kind: IntErrorKind::Empty }));
	}

	macro_rules! test_from_str_type {
		($t:ty: $name:ident) => {
			#[test]
			fn $name() {
				let mut rnd = rand::weak_rng();
				for _ in (0..1000000) {
					let i: $t = rnd.gen();
					assert_eq!(FromStr::from_str(&format!("{}", i)), Ok(Wrapper(i)));
				}
			}
		}
	}

	test_from_str_type!(u8: test_from_str_u8);
	test_from_str_type!(u16: test_from_str_u16);
	test_from_str_type!(u32: test_from_str_u32);
	test_from_str_type!(u64: test_from_str_u64);
	test_from_str_type!(i8: test_from_str_i8);
	test_from_str_type!(i16: test_from_str_i16);
	test_from_str_type!(i32: test_from_str_i32);
	test_from_str_type!(i64: test_from_str_i64);
}


#[cfg(test)]
mod display_tests {
	use super::Wrapper;
	use rand::{self, Rng};

	#[test]
	fn test_pos() {
		for i in (0i64..10000) {
			let n = Wrapper(i);
			assert_eq!(format!("{}", i), format!("{}", n));
		}
	}

	#[test]
	fn test_neg() {
		for i in (0i64..10000) {
			let n = Wrapper(-i);
			assert_eq!(format!("{}", -i), format!("{}", n));
		}
	}

	#[test]
	fn test_overflow() {
		assert_eq!("-128", format!("{}", Wrapper(-128i8)));
		assert_eq!("-32768", format!("{}", Wrapper(-32768i16)));
		assert_eq!("-2147483648", format!("{}", Wrapper(-2147483648i32)));
		assert_eq!("-9223372036854775808", format!("{}", Wrapper(-9223372036854775808i64)));
	}

	macro_rules! test_display_type {
		($t:ty: $name:ident) => {
			#[test]
			fn $name() {
				let mut rnd = rand::weak_rng();
				for _ in (0..1000000) {
					let i: $t = rnd.gen();
					assert_eq!(format!("{}", i), format!("{}", Wrapper(i)));
				}
			}
		}
	}

	test_display_type!(u8: test_display_u8);
	test_display_type!(u16: test_display_u16);
	test_display_type!(u32: test_display_u32);
	test_display_type!(u64: test_display_u64);
	test_display_type!(i8: test_display_i8);
	test_display_type!(i16: test_display_i16);
	test_display_type!(i32: test_display_i32);
	test_display_type!(i64: test_display_i64);
}