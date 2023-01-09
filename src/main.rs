#![allow(unused)]

struct Solution;
impl Solution {
	pub fn permute(nums: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let len = nums.len();
		if len == 1 {
			return vec![nums];
		}
		let mut ret = vec![];
		for i in 0..len {
			let mut cl = nums.clone();
			let rm = cl.remove(i,);
			Self::permute(cl,).iter_mut().for_each(|v| {
				v.push(rm,);
				ret.push(v.clone(),);
			},)
		}

		ret
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]];
		let mut sol = Solution::permute(vec![1, 2, 3],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![0, 1], vec![1, 0]];
		let mut sol = Solution::permute(vec![0, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![vec![1]];
		let mut sol = Solution::permute(vec![1],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
