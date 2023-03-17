// •‿•
#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn subsets(nums: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let len = nums.len();
		let p = 1 << len;
		let mut rslt = vec![vec![]; p];
		for i in 0..p {
			for j in 0..len {
				if (i >> j) & 1 == 1 {
					rslt[i].push(nums[j],);
				}
			}
		}
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![
			vec![],
			vec![1],
			vec![2],
			vec![1, 2],
			vec![3],
			vec![1, 3],
			vec![2, 3],
			vec![1, 2, 3],
		];
		let mut sol = Solution::subsets(vec![1, 2, 3],);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![], vec![0]];
		let mut sol = Solution::subsets(vec![0],);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v.last(),);
			}
		},)
	}

	#[bench]
	fn b2(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v[v.len() - 1],);
			}
		},)
	}
}

fn main() {}
