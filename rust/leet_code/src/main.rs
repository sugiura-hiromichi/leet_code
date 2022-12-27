#![allow(unused)]

struct Solution;
impl Solution {
	pub fn combination_sum(candidates: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
		let mut ret = vec![];
		for i in 0..candidates.len() {
			let mut max = i;
		}

		// q:
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![2, 2, 3], vec![7]];
		let mut sol = Solution::combination_sum(vec![2, 3, 6, 7], 7,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
		let mut sol = Solution::combination_sum(vec![2, 3, 5], 8,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans: Vec<Vec<i32,>,> = vec![];
		let mut sol = Solution::combination_sum(vec![2], 1,);
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
