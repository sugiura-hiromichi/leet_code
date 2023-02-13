// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn set_zeroes(mtrx: &mut Vec<Vec<i32,>,>,) {
		let mut set = std::collections::HashSet::new();
		for i in 0..mtrx.len() {
			for j in 0..mtrx[0].len() {
				if mtrx[i][j] == 0 {
					set.insert((i, j,),);
				}
			}
		}

		set.iter().for_each(|(i, j,)| {
			Self::set_zero(mtrx, *i, *j,);
		},)
	}

	fn set_zero(mtrx: &mut Vec<Vec<i32,>,>, i: usize, j: usize,) {
		for x in 0..mtrx.len() {
			for y in 0..mtrx[0].len() {
				if x == i || y == j {
					mtrx[x][y] = 0;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
		let mut sol = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
		Solution::set_zeroes(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
		let mut sol = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
		Solution::set_zeroes(&mut sol,);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

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

fn main() {}
