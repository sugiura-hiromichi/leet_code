// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn my_sqrt(x: i32,) -> i32 {
		let (mut l, mut r,) = (0, x,);
		while l < r {
			let mid = (l + r) / 2;
			let (rslt, flowed,) = mid.overflowing_mul(mid,);
			if !flowed && rslt < x {
				l = mid + 1;
			} else {
				r = mid;
			}
		}

		let (rslt, flowed,) = l.overflowing_mul(l,);
		if !flowed && rslt <= x {
			l
		} else {
			l - 1
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 2;
		let mut sol = Solution::my_sqrt(4,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 2;
		let mut sol = Solution::my_sqrt(8,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 46339;
		let mut sol = Solution::my_sqrt(2147395599,);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	use super::*;

	//	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			// fastest vector init
			let mut v = vec![0; 1e5 as usize];
			for i in 0..1e5 as i32 {
				v[i as usize] = i;
			}
		},)
	}
}

fn main() { assert_eq!(21 - 15 % 6, 18) }
