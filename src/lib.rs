#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `1<=heights.len()<=10^5`f ZZ
	/// - `0<=heights[i] <= 10^4`
	/// - list is guaranteed to be sorted in ascending order
	pub fn largest_rectangle_area(heights: Vec<i32,>,) -> i32 {
		let len = heights.len();

		let mut left_less = vec![0; len];
		let mut right_less = vec![0; len];
		left_less[0] = -1;
		right_less[len - 1] = len;

		for i in 1..len {
			let mut p = i as i32 - 1;
			while p >= 0 && heights[p as usize] >= heights[i] {
				p = left_less[p as usize];
			}
			left_less[i] = p;
		}

		for i in (0..len - 1).rev() {
			let mut p = i + 1;
			while p < len && heights[p] >= heights[i] {
				p = right_less[p];
			}
			right_less[i] = p;
		}

		let mut rslt = 0;
		for i in 0..len {
			let rct = heights[i] * (right_less[i] as i32 - left_less[i] - 1);
			if rct > rslt {
				rslt = rct;
			}
		}

		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let ans = 10;
		let sol = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = 4;
		let sol = Solution::largest_rectangle_area(vec![2, 4],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let ans = 10;
		let sol = Solution::largest_rectangle_area(vec![
			10, 1, 1, 1, 1, 1, 1, 1, 1, 4,
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let ans = 3;
		let sol = Solution::largest_rectangle_area(vec![2, 1, 2],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let ans = 20;
		let sol =
			Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0],);
		assert_eq!(ans, sol);
	}
}
