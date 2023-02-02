// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn plus_one(mut digs: Vec<i32,>,) -> Vec<i32,> {
		let mut i = digs.len() - 1;
		while i > 0 && digs[i] == 9 {
			digs[i] = 0;
			i -= 1;
		}

		if i == 0 && digs[0] == 9 {
			digs[0] = 0;
			digs.insert(0, 1,);
		} else {
			digs[i] += 1;
		}
		digs
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![1, 2, 4];
		let mut sol = Solution::plus_one(vec![1, 2, 3],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![4, 3, 2, 2];
		let mut sol = Solution::plus_one(vec![4, 3, 2, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![1, 0];
		let mut sol = Solution::plus_one(vec![9],);
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
