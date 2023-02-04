// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn climb_stairs(x: i32,) -> i32 {
		let mut rslt = 1;
		for i in 1..=x / 2 {
			rslt += Self::combination((x - i) as u128, i as u128,);
		}

		rslt
	}

	fn combination(total: u128, select: u128,) -> i32 {
		((total - select + 1..=total).product::<u128>() / (1..=select).product::<u128>()) as i32
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
