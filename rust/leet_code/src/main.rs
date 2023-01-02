#![allow(unused)]

struct Solution;
impl Solution {
	pub fn trap(height: Vec<i32,>,) -> i32 {
		let len = height.len();
		let (mut water, mut l, mut r, mut max,) = (0, 0, len - 1, 0,);

		while l < r {
			max = max.max(height[l].min(height[r],),);
			if height[l] > height[r] {
				r -= 1;
				water += 0.max(max - height[r],);
			} else {
				l += 1;
				water += 0.max(max - height[l],);
			}
		}
		water
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 6;
		let mut sol = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 8;
		let mut sol = Solution::trap(vec![5, 8, 9, 4, 9, 6, 1, 4],);
		assert_eq!(ans, sol);
	}

	// FAIL: sol=5
	#[test]
	fn test_3() {
		let mut ans = 12;
		let mut sol = Solution::trap(vec![2, 8, 5, 5, 6, 1, 7, 4, 5],);
		assert_eq!(ans, sol);
	}
}

fn main() {
	// water 0, i 5, l 1, r 8
	// water 4, i 7, l 1, r 8
	Solution::trap(vec![2, 8, 5, 5, 6, 1, 7, 4, 5],);
}
