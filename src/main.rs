#![allow(unused)]

struct Solution;
impl Solution {
	pub fn rotate(mtrx: &mut Vec<Vec<i32,>,>,) {
		let size = mtrx.len();
		for i in 0..(size + 1) / 2 {
			let len = size - 1 - i * 2;
			for l in 0..len {
				let tmp = mtrx[i][i + l];
				mtrx[i][i + l] = mtrx[len + i - l][i];
				mtrx[len + i - l][i] = mtrx[len + i][len + i - l];
				mtrx[len + i][len + i - l] = mtrx[i + l][len + i];
				mtrx[i + l][len + i] = tmp;
			}
		}
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	fn vectorize(ary: &[&[i32]],) -> Vec<Vec<i32,>,> { ary.iter().map(|a| a.to_vec(),).collect() }

	#[test]
	fn test_1() {
		let mut ans = vectorize(&[&[7, 4, 1,], &[8, 5, 2,], &[9, 6, 3,],],);
		let mut sol = vectorize(&[&[1, 2, 3,], &[4, 5, 6,], &[7, 8, 9,],],);
		Solution::rotate(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vectorize(&[&[15, 13, 2, 5,], &[14, 3, 4, 1,], &[12, 6, 8, 9,], &[16, 7, 10, 11,],],);
		let mut sol = vectorize(&[&[5, 1, 9, 11,], &[2, 4, 8, 10,], &[13, 3, 6, 7,], &[15, 14, 12, 16,],],);
		Solution::rotate(&mut sol,);
		assert_eq!(ans, sol);
	}
}

fn main() {}
