#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `1<=heights.len()<=10^5`
	/// - `0<=heights[i] <= 10^4`
	/// - list is guaranteed to be sorted in ascending order
	pub fn largest_rectangle_area(heights: Vec<i32,>,) -> i32 {
		let len = heights.len() - 1;
		let mut rslt = 0;

		// NOTE: When units are max, left edge index(assume `left`) satisfies
		// `(left==0 || heights[left-1]<heights[left]) &&
		// (left==heights.len()-1 || heights[left]<=heights[left+1])`
		// collect indices which satisfies above condition
		for i in 0..heights.len() {
			if i == 0 || heights[i - 1] < heights[i]
			//&& (i == heights.len() - 1 || heights[i] <= heights[i + 1])
			{
				let mut j = i;
				let mut min = heights[i];
				loop {
					if heights[j] < min {
						if (j - i) as i32 * min > rslt {
							rslt = (j - i) as i32 * min;
						}
						min = heights[j];
					}
					if j == len {
						if (j - i + 1) as i32 * min > rslt {
							rslt = (j - i + 1) as i32 * min;
						}
						break;
					}
					j += 1;
				}
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
}
