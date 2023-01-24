#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn can_jump(nums: Vec<i32,>,) -> bool {
		let len = nums.len() - 1;
		let mut i = 0;
		while i < len {
			let mut max = i;
			for j in i + 1..=i + nums[i] as usize {
				if j == len {
					return true;
				}
				if nums[j] != 0 {
					if nums[max] as usize + max < j + nums[j] as usize {
						max = j;
					}
				}
			}

			if max == i {
				return false;
			}
			i = max;
		}
		true
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
}

fn main() {}
