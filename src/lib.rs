#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `rows == matrix.length`
	/// - `cols == matrix.length`
	/// - `1 <= row` `cols <= 200`
	/// - `matrix[i][j] = '0' | '1'`
	pub fn maximal_rectangle(matrix: Vec<Vec<char,>,>,) -> i32 {
		let (r_max, c_max,) = (matrix.len(), matrix[0].len(),);
		let (mut r, mut c,) = (0, 0,);

		let mut rslt = 0;
		while r < r_max {
			while c < c_max {
				let mut heights = vec![];
				if matrix[r][c] == '1' {
					while c < c_max && matrix[r][c] == '1' {
						let mut h = 1;
						while r + h < r_max && matrix[r + h][c] == '1' {
							h += 1;
						}
						heights.push(h,);
						c += 1;
					}
					println!("{heights:?}");
					rslt = Self::largest_rectangle_area(&heights,).max(rslt,);
					if (r_max - r) * c_max <= rslt {
						break;
					}
				}
				c += 1;
			}

			c = 0;
			r += 1;
		}

		rslt as i32
	}

	/// # Parameters
	///
	/// - `1<=heights.len()<=10^5`
	/// - `0<=heights[i] <= 10^4`
	/// - list is guaranteed to be sorted in ascending order
	pub fn largest_rectangle_area(heights: &Vec<usize,>,) -> usize {
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
			let rct =
				heights[i] * (right_less[i] - (left_less[i] + 1) as usize);
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
		let ans = 6;
		let sol = Solution::maximal_rectangle(vec![
			vec!['1', '0', '1', '0', '0'],
			vec!['1', '0', '1', '1', '1'],
			vec!['1', '1', '1', '1', '1'],
			vec!['1', '0', '0', '1', '0'],
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = 0;
		let sol = Solution::maximal_rectangle(vec![vec!['0']],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let ans = 1;
		let sol = Solution::maximal_rectangle(vec![vec!['1']],);
		assert_eq!(ans, sol);
	}
}
