#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn spiral_order(matrix: Vec<Vec<i32,>,>,) -> Vec<i32,> {
		let (row, col,) = (matrix.len(), matrix[0].len(),);
		let mut rslt = vec![];
		for i in 0..(row.min(col,) + 1) / 2 {
			let (min, max_row, max_col,) = (i, row - i, col - i,);
			if max_row - min == 1 {
				matrix[min][min..max_col].iter().for_each(|&i| rslt.push(i,),);
				break;
			}
			if max_col - min == 1 {
				matrix[min..max_row].iter().for_each(|v| rslt.push(v[min],),);
				break;
			}

			for j in min..max_col - 1 {
				//top row
				rslt.push(matrix[min][j],);
			}
			for j in min..max_row - 1 {
				//right col
				rslt.push(matrix[j][max_col - 1],);
			}
			for j in (min + 1..max_col).rev() {
				//bottom row
				rslt.push(matrix[max_row - 1][j],);
			}
			for j in (min + 1..max_row).rev() {
				//left col
				rslt.push(matrix[j][min],);
			}
		}

		//if row % 2 == 1 && row == col { panic!("hello"); rslt.push(matrix[row / 2][row / 2],); }
		rslt
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
		let mut sol = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
		let mut sol =
			Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![6, 9, 7];
		let mut sol = Solution::spiral_order(vec![vec![6, 9, 7]],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
