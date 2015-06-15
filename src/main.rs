#![feature(custom_derive)]
#![feature(test)]
#![feature(zero_one)]
#[cfg(test)] extern crate test;

use std::fmt::{self, Display};
use std::ops::{Not, Add};
use std::mem;
use std::num::{Zero, One};
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

impl<T> Display for Wrapper<T> where T: Not<Output=T> + Add<Output=T> + AsU64 + Zero + One + Display + Ord + Copy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let is_positive = self.n >= T::zero();
		let mut n: u64 = if ! is_positive {
			(!self.n + T::one()).as_u64()
		} else {
			self.n.as_u64()
		};
		let mut buf: [u8; 20] = unsafe { mem::uninitialized() };
		let mut curr = buf.len() as isize;
		let buf_ptr = &mut buf[0] as *mut u8;
		let lut_ptr = &DEC_DIGITS_LUT as *const u8;

		unsafe {
			// if T::max_u64() >= 100000000 {
			// 	while n >= 100000000 {
			// 		let rem = (n % 100000000) as isize;
			// 		n /= 100000000;
			//         let b = rem / 10000;
			//         let c = rem % 10000;
			//         let d1 = (b / 100) << 1;
			//         let d2 = (b % 100) << 1;
			//         let d3 = (c / 100) << 1;
			//         let d4 = (c % 100) << 1;
			// 		curr -= 8;
			// 		ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
			// 		ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
			// 		ptr::copy_nonoverlapping(lut_ptr.offset(d3), buf_ptr.offset(curr + 4), 2);
			// 		ptr::copy_nonoverlapping(lut_ptr.offset(d4), buf_ptr.offset(curr + 6), 2);
			// 	}
			// }

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

fn main() {
	unsafe {
		let lut_ptr = &DEC_DIGITS_LUT as *const u8;
		for i in (0usize..DEC_DIGITS_LUT.len()) {
			if i != 0 && i % 10 == 0 {
				println!("");
			}
			print!("{:?} {:?}, ", *lut_ptr.offset(i as isize), *lut_ptr.offset(i as isize) as char);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Wrapper;
	use test;
	use std::fmt::Write;
	use std::{i8, i16, i32, i64};

	#[bench]
	fn _warmup(b: &mut test::Bencher) {
		b.iter(|| 1)
	}

	#[bench]
	fn stdlib_08(b: &mut test::Bencher) {
		let mut n = 1i8;
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i8;
			while i <= i8::MAX / 2 {
				i *= 2;
				n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn stdlib_16(b: &mut test::Bencher) {
		let mut n = 1i16;
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i16;
			while i <= i16::MAX / 2 {
				i *= 2;
				n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn stdlib_32(b: &mut test::Bencher) {
		let mut n = 1i32;
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1;
			while i <= i32::MAX / 2 {
				i *= 2;
				n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn stdlib_64(b: &mut test::Bencher) {
		let mut n = 1i64;
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1;
			while i <= i64::MAX / 2 {
				i *= 2;
				n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn new_08(b: &mut test::Bencher) {
		let mut n = Wrapper{n: 1i8};
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i8;
			while i <= i8::MAX / 2 {
				i *= 2;
				n.n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n.n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn new_16(b: &mut test::Bencher) {
		let mut n = Wrapper{n: 1i16};
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i16;
			while i <= i16::MAX / 2 {
				i *= 2;
				n.n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n.n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn new_32(b: &mut test::Bencher) {
		let mut n = Wrapper{n: 1i32 << 16};
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i32;
			while i <= i32::MAX / 2 {
				i *= 2;
				n.n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n.n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

	#[bench]
	fn new_64(b: &mut test::Bencher) {
		let mut n = Wrapper{n: 1i64 << 16};
		let mut buf = String::with_capacity(100);
		b.iter(|| {
			let mut i = 1i64;
			while i <= i64::MAX / 2 {
				i *= 2;
				n.n = i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
				n.n = -i;
				buf.clear();
				buf.write_fmt(format_args!("{}", n));
			}
		})
	}

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
	fn test_big() {
		let mut n = Wrapper{n: 1i64};
		for i in (0i64..100000) {
			n.n = i * -331;
			assert_eq!(format!("{}", i * -331), format!("{}", n));
		}
	}
}