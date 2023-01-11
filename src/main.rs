#![allow(unused)]

struct Solution;
impl Solution {
	pub fn rotate(mtrx: &mut Vec<Vec<i32,>,>,) {
		let size = mtrx.len();
		for i in 0..(size + 1) / 2 {
			Self::swap(mtrx, size, i,);
		}
	}

	fn swap(mtrx: &mut Vec<Vec<i32,>,>, size: usize, swapped: usize,) {
		let len = size - 1 - swapped * 2;
		let i0 = (swapped, swapped,);
		let i1 = (i0.0, len + i0.1,);
		let i2 = (i1.1, i1.1,);
		let i3 = (i1.1, i1.0,);
		for l in 0..len {
			//(mtrx[i0.0][i0.1 + l], mtrx[i1.0 + l][i1.1], mtrx[i2.0][i2.1 - l], mtrx[i3.0 - l][i3.1],) = (mtrx[i3.0 - l][i3.1], mtrx[i0.0][i0.1 + l], mtrx[i1.0 + l][i1.1], mtrx[i2.0][i2.1 - l],);

         let tmp=mtrx[i0.0][i0.1+l];
         mtrx[i0.0][i0.1+l]=mtrx[i3.0-l][i3.1];
         mtrx[i3.0-l][i3.1]=mtrx[i2.0][i2.1-l];
         mtrx[i2.0][i2.1-l]=mtrx[i1.0+l][i1.1];
         mtrx[i1.0+l][i1.1]=tmp;
         
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
