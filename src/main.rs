#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn merge(mut nums: Vec<Vec<i32,>,>,) -> Vec<Vec<i32,>,> {
		nums.sort();
		let mut rslt = vec![vec![nums[0][0], nums[0][1]]];
		let mut rs_i = 0;
		for i in 1..nums.len() {
			if rslt[rs_i][1] < nums[i][0] {
				rslt.push(nums[i].clone(),);
				rs_i += 1;
			} else {
				if rslt[rs_i][1] < nums[i][1] {
					rslt[rs_i][1] = nums[i][1];
				}
			}
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
		let mut ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
		let mut sol = Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1, 5]];
		let mut sol = Solution::merge(vec![vec![1, 4], vec![4, 5]],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
