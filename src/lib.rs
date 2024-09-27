#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `rows == matrix.length`
	/// - `cols == matrix.length`
	/// - `1 <= row` `cols <= 200`
	/// - `matrix[i][j] = '0' | '1'`
	pub fn maximal_rectangle(matrix: Vec<Vec<char,>,>,) -> i32 { todo!() }
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
