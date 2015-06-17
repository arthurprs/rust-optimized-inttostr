#![feature(custom_derive)]
#![feature(test)]
#![feature(zero_one)]
#[cfg(test)] extern crate test;
#[cfg(test)] extern crate strconv;
extern crate rand;

use std::fmt::{self, Display};
use std::ops::Not;
use std::mem;
use std::num::{Zero, One, Wrapping};
use std::str;
use std::ptr;

trait AsU64 {
	fn as_u64(self) -> u64;
	fn max_u64() -> u64;
}

macro_rules! impl_AsU64 {
	($($t:ident)*) => ($(impl AsU64 for $t {
		#[inline(always)]
        fn as_u64(self) -> u64 { self as u64 }
		#[inline(always)]
        fn max_u64() -> u64 { Self::max_value() as u64 }
    })*)
}

impl_AsU64! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

pub struct Wrapper<T> {
    n: T
}

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

impl<T> Display for Wrapper<T> where
			T: Not<Output=T> + AsU64 + Zero + One + Display + Ord + Copy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let is_positive = self.n >= T::zero();
		let mut n: u64 = if ! is_positive {
			let Wrapping(n) = Wrapping((!self.n).as_u64()) + Wrapping(1);
			n
		} else {
			self.n.as_u64()
		};
		let mut buf: [u8; 20] = unsafe { mem::uninitialized() };
		let mut curr = buf.len() as isize;
		let buf_ptr = &mut buf[0] as *mut u8;
		let lut_ptr = &DEC_DIGITS_LUT as *const u8;

		unsafe {
			if T::max_u64() >= 10000 {
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

			// n can be safelly treated as isize from this point on
			// this increases performance on 32bit systems
			let mut n = n as isize;

			while n >= 100 {
				let d1 = (n % 100) << 1;
				n /= 100;
				curr -= 2;
				ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
			}

			if n < 10 {
				curr -= 1;
				*buf_ptr.offset(curr) = (n as u8) + 48;
			} else {
				let d1 = (n as isize) << 1;
				curr -= 2;
				ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
			}
		}
		
        f.pad_integral(is_positive, "", unsafe { str::from_utf8_unchecked(&buf[curr as usize..]) })
    }
}

#[cfg(not(test))]
fn main() {
	use rand::{Rng, SeedableRng, StdRng};
	const SEED: &'static [usize] = &[0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
	let mut rng: StdRng = SeedableRng::from_seed(SEED);
	let mut hist = [0u32; 21];

	for _ in (0..10000) {
		let x: f64 = rng.gen();
		let x = x.powf(25f64);
		hist[((x * 2_147_483_647f64) as u32).to_string().len()] += 1;
	}

	println!("{:?}", hist);
}

#[cfg(test)]
pub mod bench;

#[cfg(test)]
mod tests {
	use super::Wrapper;
	use rand::{self, Rng};

	#[test]
	fn test_pos() {
		let mut n = Wrapper{n: 1i64};
		for i in (0i64..10000) {
			n.n = i;
			assert_eq!(format!("{}", i), format!("{}", n));
		}
	}

	#[test]
	fn test_neg() {
		let mut n = Wrapper{n: 1i64};
		for i in (0i64..10000) {
			n.n = -i;
			assert_eq!(format!("{}", -i), format!("{}", n));
		}
	}

	#[test]
	fn test_overflow() {
		assert_eq!("-128", format!("{}", Wrapper{n: -128i8}));
		assert_eq!("-32768", format!("{}", Wrapper{n: -32768i16}));
		assert_eq!("-2147483648", format!("{}", Wrapper{n: -2147483648i32}));
		assert_eq!("-9223372036854775808", format!("{}", Wrapper{n: -9223372036854775808i64}));
	}

	macro_rules! test_type {
		($t:ty: $name:ident) => {
			#[test]
			fn $name() {
				let mut rnd = rand::weak_rng();
				for _ in (0..100000) {
					let i: $t = rnd.gen();
					assert_eq!(format!("{}", i), format!("{}", Wrapper{n: i}));
				}
			}
		}
	}

	test_type!(u8: test_u8);
	test_type!(u16: test_u16);
	test_type!(u32: test_u32);
	test_type!(u64: test_u64);
	test_type!(i8: test_i8);
	test_type!(i16: test_i16);
	test_type!(i32: test_i32);
	test_type!(i64: test_i64);
}