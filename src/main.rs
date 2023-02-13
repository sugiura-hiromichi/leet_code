// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn search_matrix(matrix: Vec<Vec<i32,>,>, target: i32,) -> bool {
		let (mut a, mut b,) = (0, matrix.len(),);
		while a < b {
			let mid = (a + b) / 2;
			if matrix[mid][0] < target {
				a = mid + 1;
			} else {
				b = mid;
			}
		}

		if a < matrix.len() && matrix[a][0] == target {
			return true;
		}

		let row = if a == 0 { 0 } else { a - 1 };
		a = 0;
		b = matrix[0].len();
		while a < b {
			let mid = (a + b) / 2;
			if matrix[row][mid] < target {
				a = mid + 1;
			} else {
				b = mid;
			}
		}

		a < matrix[0].len() && matrix[row][a] == target
			|| matrix[row][if a == 0 { 0 } else { a - 1 }] == target
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = true;
		let mut sol = Solution::search_matrix(
			vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
			3,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = false;
		let mut sol = Solution::search_matrix(
			vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
			13,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = false;
		let mut sol = Solution::search_matrix(vec![vec![1]], 0,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = false;
		let mut sol = Solution::search_matrix(vec![vec![1]], 2,);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

	//	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			// fastest vector init
			let mut v = vec![0; 1e5 as usize];
			for i in 0..1e5 as i32 {
				v[i as usize] = i;
			}
		},)
	}
}

fn main() {}
