#![allow(unused)]

/// this is doc comment.
/// `hello` **how are you**
struct Solution;
impl Solution {
	pub fn first_missing_positive(nums: Vec<i32,>,) -> i32 {
		if nums.len() == 1 {
			return if nums[0] == 1 { 2 } else { 1 };
		}

		let mut nums = nums;
		nums.sort();
		let (mut l, mut r,) = (0, nums.len(),);
		if nums[r - 1] <= 0 {
			return 1;
		}

		// find smallest positive number
		while nums[l] <= 0 {
			if l >= r {
				return 1;
			}
			let mid = (l + r) / 2;
			if nums[mid] <= 0 {
				l = mid + 1;
			} else {
				r = mid;
			}
		}

		if nums[l] > 1 {
			return 1;
		}

		while l + 1 < nums.len() {
			if nums[l + 1] - nums[l] > 1 {
				return nums[l] + 1;
			}
			l += 1;
		}

		nums[l] + 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 3;
		let mut sol = Solution::first_missing_positive(vec![1, 2, 0],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 2;
		let mut sol = Solution::first_missing_positive(vec![3, 4, -1, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 1;
		let mut sol = Solution::first_missing_positive(vec![7, 8, 9, 11, 12],);
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
