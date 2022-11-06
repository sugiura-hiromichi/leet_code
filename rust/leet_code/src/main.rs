#![allow(dead_code)]
struct Solution;
impl Solution {
	pub fn three_sum_closest(nums: Vec<i32,>, target: i32,) -> i32 {
		let mut nearest = nums[0] + nums[1] + nums[2];
		let mut nums = nums;
		nums.sort();

		for i in 0..nums.len() - 2 {
			let Some((mut j,mut k))=(i==0 || nums[i]!=nums[i-1]).then(|| (i+1,nums.len()-1)) else{
            continue;
         };

			while j < k {
				let tmp_sum = nums[i] + nums[j] + nums[k];

				if (tmp_sum - target).abs() < (nearest - target).abs() {
					nearest = tmp_sum;
				}

				if tmp_sum == target {
					return target;
				}
				if tmp_sum > target {
					k -= 1;
				} else {
					j += 1;
				}
			}
		}

		nearest
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
	}

	#[test]
	fn test_2() {
		assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
	}

	#[test]
	fn test_3() {
		assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
	}
}
