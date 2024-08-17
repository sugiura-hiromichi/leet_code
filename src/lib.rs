#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `1 <= nums.len() <= 5000`
	/// - `-10^4 <= nums[i] <= 10^4`
	/// - `nums` is sorted in non-decreasing order, then rotated at an unknown
	///   pivot
	/// - `-10^4 <= target <= 10^4`
	pub fn search(nums: Vec<i32,>, target: i32,) -> bool {
		let last = nums.len() - 1;

		match target.cmp(&nums[0],) {
			// target < nums[0]
			std::cmp::Ordering::Less => {
				let mut i = last;
				loop {
					match target.cmp(&nums[i],) {
						std::cmp::Ordering::Less if i != 0 => i -= 1,
						_ => return target == nums[i],
					}
				}
			},
			std::cmp::Ordering::Equal => true,
			std::cmp::Ordering::Greater => {
				let mut i = 0;
				loop {
					match target.cmp(&nums[i],) {
						std::cmp::Ordering::Greater if i != last => i += 1,
						_ => return target == nums[i],
					}
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let ans = true;
		let sol = Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = false;
		let sol = Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let ans = true;
		let sol = Solution::search(vec![1, 0, 1, 1, 1], 0,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let ans = true;
		let sol = Solution::search(vec![1, 1, 1, 0, 1], 0,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let ans = true;
		let sol = Solution::search(vec![1, 1, 0, 1, 1], 0,);
		assert_eq!(ans, sol);
	}
}
