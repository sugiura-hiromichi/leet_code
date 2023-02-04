// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	// fibonacci
	pub fn climb_stairs(x: i32,) -> i32 {
		if x < 3 {
			return x;
		}

		let (mut one_bef, mut two_bef, mut sum,) = (2, 1, 0,);
		for i in 2..x {
			sum = one_bef + two_bef;
			two_bef = one_bef;
			one_bef = sum;
		}

		sum
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 2;
		let mut sol = Solution::climb_stairs(2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 5;
		let mut sol = Solution::climb_stairs(4,);
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
