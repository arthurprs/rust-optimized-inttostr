use std::fmt::{self, Display};
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

pub struct Wrapper<T>(pub T);

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
	})*);
}

impl_Display!(i8, u8, i16, u16, i32, u32: as_u32);
impl_Display!(i64, u64: as_u64);
#[cfg(target_pointer_width = "32")]
impl_Display!(isize, usize: as_u32);
#[cfg(target_pointer_width = "64")]
impl_Display!(isize, usize: as_u64);

