#![allow(unused)]

struct Solution;
impl Solution {
	pub fn jump(nums: Vec<i32,>,) -> i32 {
		let len = nums.len() - 1;
		let mut i = 0;
		let mut ret = 0;
		while i < len {
			ret += 1;
			if i + nums[i] as usize >= len {
				break;
			}
			let jump = (len - i).min(nums[i] as usize,);
			let (mut max, mut tmp_i,) = (0, 0,);
			for j in 1..=jump {
				if max < j as i32 + nums[i + j] {
					max = j as i32 + nums[i + j];
					tmp_i = j;
				}
			}

			i += tmp_i;
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
