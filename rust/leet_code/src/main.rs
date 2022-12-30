#![allow(unused)]

struct Solution;
impl Solution {
	pub fn trap(height: Vec<i32,>,) -> i32 {
		let len = height.len();
		let hold_mxp = {
			let mut ret = 0;
			for i in 1..len {
				if height[ret] < height[i] {
					ret = i;
				}
			}
			ret
		};
		let (mut mxp, mut water,) = (hold_mxp, 0,);

		// d: at first, sum up range of 0..mxp
		while mxp > 1 {
			let ano = {
				let mut ret = 0;
				for i in 1..mxp {
					if height[ret] < height[i] {
						ret = i;
					}
				}
				ret
			};
			for i in ano..mxp {
				water += height[ano] - height[i];
			}
			mxp = ano;
		}

		// d: second, sum up range of mxp..len
		mxp = hold_mxp + 1;
		while mxp < len {
			let ano = {
				let mut ret = mxp;
				for i in mxp..len {
					if height[ret] <= height[i] {
						ret = i;
					}
				}
				ret
			};
			for i in mxp..ano {
				water += height[ano] - height[i];
			}

			mxp = ano + 1;
		}

		water
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 6;
		let mut sol = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],);
		assert_eq!(ans, sol);
	}

	//FAIL: infinite loop
	#[test]
	fn test_2() {
		let mut ans = 0;
		let mut sol = Solution::trap(vec![3, 4, 1, 1],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 3;
		let mut sol = Solution::trap(vec![9, 7, 8, 11, 12],);
		assert_eq!(ans, sol);
	}
}

fn main() {}
