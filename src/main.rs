// •‿•
#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn subsets(nums: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let (mut start, mut end,) = (0, 0,);
		let mut rslt = nums.iter().fold(vec![], |mut acc, &x| {
			acc.push(vec![x],);
			end += 1;
			acc
		},);

		let len = nums.len();
		while rslt[end - 1].len() < len {
			for i in start..end {
				let ith = rslt[i].clone();
				let mut pos =
					nums.iter().enumerate().find(|&x| *x.1 == ith[ith.len() - 1],).unwrap().0;
				while pos < len - 1 {
					pos += 1;
					let mut tmp = ith.clone();
					tmp.push(nums[pos],);
					rslt.push(tmp,);
				}
			}
			start = end;
			end = rslt.len();
		}

		rslt.push(vec![],);
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
