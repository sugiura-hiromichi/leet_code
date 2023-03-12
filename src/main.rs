// •‿•
#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn combine(n: i32, k: i32,) -> Vec<Vec<i32,>,> {
		let mut rslt = vec![];
		for i in 1..=n - k + 1 {
			rslt.push(vec![i],);
		}

		for i in 1..k {
			let mut cl = vec![];
			rslt.iter().for_each(|v| {
				for j in v[i as usize - 1] + 1..n - k + i + 2 {
					let mut v = v.clone();
					v.push(j,);
					cl.push(v,);
				}
			},);
			rslt = cl;
		}
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]];
		let mut sol = Solution::combine(4, 2,);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1]];
		let mut sol = Solution::combine(1, 1,);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn range_test() {
		for i in 5..=4 {
			assert!(true);
		}
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
			let v = vec![0; 10000];
			for i in 0..1000 {
				black_box(v.last(),);
			}
		},)
	}

	#[bench]
	fn b2(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 10000];
			for i in 0..1000 {
				black_box(v[v.len() - 1],);
			}
		},)
	}
}

fn main() {}
