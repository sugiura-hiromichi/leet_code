// •‿•
#![allow(unused)]

use std::fmt::format;

struct Solution;
impl Solution {
	pub fn get_permutation(n: i32, k: i32,) -> String {
		let (n, mut k,) = (n as usize, k as usize - 1,);

		let mut asc = vec![1; n]; // asc stand for ascending
		let mut frac = 1;
		for i in 2..=n {
			frac *= i;
			asc[i - 1] = i;
		}

		let mut rslt = "".to_string();
		for i in 0..n {
			frac /= n - i;
			//println!("k / tmp | {k} / {tmp}");
			rslt.push_str(&asc.remove(k / frac,).to_string(),);
			k %= frac;
		}
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "213";
		let mut sol = Solution::get_permutation(3, 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "2314";
		let mut sol = Solution::get_permutation(4, 9,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = "123";
		let mut sol = Solution::get_permutation(3, 1,);
		assert_eq!(ans, sol);
	}
}

fn main() {}
