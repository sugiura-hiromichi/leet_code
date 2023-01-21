#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	// I think DP is recurrence formula
	pub fn max_sub_array(n: Vec<i32,>,) -> i32 {
		let len = n.len();
		let mut dp = vec![0; len];
		dp[0] = n[0];
		let mut max = dp[0];

		for i in 1..len {
			dp[i] = n[i] + if dp[i - 1] > 0 { dp[i - 1] } else { 0 };
			max = max.max(dp[i],);
		}
		max
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 6;
		let mut sol = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = -1;
		let mut sol = Solution::max_sub_array(vec![-1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 1;
		let mut sol = Solution::max_sub_array(vec![-2, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = 2;
		let mut sol = Solution::max_sub_array(vec![0, -3, -2, -3, -2, 2, -3, 0, 1, -1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = 23;
		let mut sol = Solution::max_sub_array(vec![5, 4, -1, 7, 8],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
