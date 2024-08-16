#![allow(dead_code)]

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - `1 <= nums.len() <= 3*10^4`
	/// - `-10^4 <= nums[i] <= 10^4`
	/// - `nums` is sorted in non-decreasing order.
	pub fn remove_duplicates(nums: &mut Vec<i32,>,) -> i32 {
		let mut seq = 0;
		let mut idx = 0;

		loop {
			//evaluation

			if seq >= 2 {
				nums.remove(idx,);
			} else {
				idx += 1;
			}

			//update

			if nums.len() == idx {
				break;
			}

			if nums[idx] == nums[idx - 1] {
				seq += 1;
			} else {
				seq = 0;
			}
		}
		idx as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	//	#[test]
	//	fn test_1() {
	//		let ans = true;
	//		let sol = Solution::remove_duplicates(
	//			vec![
	//				vec!['A', 'B', 'C', 'E'],
	//				vec!['S', 'F', 'C', 'S'],
	//				vec!['A', 'D', 'E', 'E'],
	//			],
	//			"ABCCED".to_string(),
	//		);
	//		assert_eq!(ans, sol);
	//	}
	//
	//	#[test]
	//	fn test_2() {
	//		let ans = true;
	//		let sol = Solution::remove_duplicates(
	//			vec![
	//				vec!['A', 'B', 'C', 'E'],
	//				vec!['S', 'F', 'C', 'S'],
	//				vec!['A', 'D', 'E', 'E'],
	//			],
	//			"SEE".to_string(),
	//		);
	//		assert_eq!(ans, sol);
	//	}

	#[test]
	fn test_3() {
		let ans = &mut vec![1, 1, 2, 2, 3];
		let sol = &mut vec![1, 1, 1, 2, 2, 3];

		let len = Solution::remove_duplicates(sol,) as usize;
		assert_eq!(len, ans.len());
		for i in 0..len {
			assert_eq!(ans[i], sol[i]);
		}
	}

	#[test]
	fn test_4() {
		let ans = &mut vec![0, 0, 1, 1, 2, 3, 3];
		let sol = &mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

		let len = Solution::remove_duplicates(sol,) as usize;
		assert_eq!(len, ans.len());
		for i in 0..len {
			assert_eq!(ans[i], sol[i]);
		}
	}
}
