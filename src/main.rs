// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn is_number(s: String,) -> bool {
		// q:
		let len = s.len();
		let mut rslt = true;
		let mut req_int = false;
		let mut was_dig = false;
		let mut already_sci = false;

		for (i, c,) in s.chars().enumerate() {
			if c.is_numeric() {
				was_dig = true;
			} else {
				if c == '+' || c == '-' {
					rslt = (i == 0 || (i > 0 && (&s[i - 1..i] == "e" || &s[i - 1..i] == "E")))
						&& i + 1 < len && match s.chars().nth(i + 1,) {
						Some(c,) if c.is_numeric() || c == '.' => true,
						_ => false,
					};
				} else if c == 'e' || c == 'E' {
					rslt = !already_sci
						&& i + 1 < len && match s.chars().nth(i + 1,) {
						Some(c,) if c.is_numeric() || c == '.' || c == '+' || c == '-' => true,
						_ => false,
					} && i != 0;
					req_int = true;
					already_sci = true;
				} else if c == '.' {
					rslt = (was_dig
						|| (i < len - 1 && s.chars().nth(i + 1,).unwrap().is_numeric()))
						&& !req_int;
					req_int = true;
				} else {
					rslt = false;
				}

				was_dig = false;
			}

			if rslt == false {
				break;
			}
		}

		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = true;
		let mut sol = Solution::is_number("0".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = false;
		let mut sol = Solution::is_number("e".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = true;
		let mut sol = Solution::is_number("+.8".to_string(),);
		assert_eq!(ans, sol);
	}
	#[test]
	fn test_4() {
		let mut ans = true;
		let mut sol = Solution::is_number("3.".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = false;
		let mut sol = Solution::is_number("9e3e2".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_6() {
		let mut ans = true;
		let mut sol = Solution::is_number("46.e+3".to_string(),);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	use super::*;

	//	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			// fastest vector init
			let mut v = vec![0; 1e5 as usize];
			for i in 0..1e5 as i32 {
				v[i as usize] = i;
			}
		},)
	}
}

fn main() { assert_eq!(21 - 15 % 6, 18) }
