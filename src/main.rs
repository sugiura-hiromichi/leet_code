// •‿•
#![allow(unused)]
struct Solution;
impl Solution {
	pub fn generate_matrix(n: i32,) -> Vec<Vec<i32,>,> {
		let mut rslt = vec![vec![0; n as usize]; n as usize];
		let mut e = 1;
		for i in 0..n as usize / 2 {
			let len = n as usize - 2 * i - 1;
			//top row
			for j in 0..len {
				rslt[i][i + j] = e;
				e += 1;
			}
			//right col
			for j in 0..len {
				rslt[i + j][len + i] = e;
				e += 1
			}
			//bottom row
			for j in 0..len {
				rslt[len + i][len + i - j] = e;
				e += 1;
			}
			//left col
			for j in 0..len {
				rslt[len + i - j][i] = e;
				e += 1;
			}
		}

		if n % 2 == 1 {
			rslt[n as usize / 2][n as usize / 2] = e;
		}
		rslt
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
		let mut sol = Solution::generate_matrix(3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1]];
		let mut sol = Solution::generate_matrix(1,);
		assert_eq!(ans, sol);
	}
}

fn main() {}
