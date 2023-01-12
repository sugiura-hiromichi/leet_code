#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn my_pow(x: f64, mut n: i32,) -> f64 {
		let mut ret = 1f64;
		if n > 0 {
			while n != 0 {
				let mut tmp = x;
				let mut pow_2_lim = 1;
				while pow_2_lim < n / 2 {
					pow_2_lim *= 2;
					tmp *= tmp;
				}
				ret *= tmp;
				n -= pow_2_lim;
			}
		} else {
			while n != 0 {
				let mut tmp = x;
				let mut pow_2_lim = -1;
				while pow_2_lim > n / 2 {
					pow_2_lim *= 2;
					tmp *= tmp;
				}
				ret /= tmp;
				n -= pow_2_lim;
			}
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
		let mut ans = 1024f64;
		let mut sol = Solution::my_pow(2f64, 10,);
		assert!((ans - sol).abs() < EPSILON);
	}

	#[test]
	fn test_2() {
		let mut ans = 9.261;
		let mut sol = Solution::my_pow(2.1, 3,);
		assert!((ans - sol).abs() < EPSILON);
	}

	#[test]
	fn test_3() {
		let mut ans = 0.25;
		let mut sol = Solution::my_pow(2f64, -2,);
		assert_eq!(ans, sol);
		assert!((ans - sol).abs() < EPSILON);
	}
}

fn main() {}
