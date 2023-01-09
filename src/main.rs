#![allow(unused)]

struct Solution;
impl Solution {
	pub fn jump(nums: Vec<i32,>,) -> i32 {
		let (mut fur, mut end, mut ret, l1,) = (0, 0, 0, nums.len(),);
		for i in 0..l1 - 1 {
			fur = fur.max(i + nums[i] as usize,);
			if i == end {
				ret += 1;
				end = fur;
			}
		}
		ret
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 2;
		let mut sol = Solution::jump(vec![2, 3, 1, 1, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 2;
		let mut sol = Solution::jump(vec![2, 3, 0, 0, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 2;
		let mut sol = Solution::jump(vec![1, 2, 0, 1],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
