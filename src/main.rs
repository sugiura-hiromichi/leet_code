#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	// p: just scan 0 and element which can jump beyond 0. if such element not found, return false
	pub fn can_jump(nums: Vec<i32,>,) -> bool {
		let mut n = 1;
		for i in (0..nums.len() - 1).rev() {
			n = if nums[i] < n { n + 1 } else { 1 };
		}
		n == 1
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = true;
		let mut sol = Solution::can_jump(vec![2, 3, 1, 1, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = false;
		let mut sol = Solution::can_jump(vec![3, 2, 1, 0, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = true;
		let mut sol = Solution::can_jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn split_tes() { let empty: &[i32] = &[]; }
}

fn main() {}
