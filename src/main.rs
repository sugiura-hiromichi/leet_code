// •‿•
#![allow(unused)]
struct Solution;
impl Solution {
	pub fn length_of_last_word(s: String,) -> i32 {
		let mut i = s.len();
		while i > 0 && &s[i - 1..i] == " " {
			i -= 1;
		}

		let tmp = i;
		while i > 0 && &s[i - 1..i] != " " {
			i -= 1;
		}
		(tmp - i) as i32
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 5;
		let mut sol = Solution::length_of_last_word("Hello World".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 4;
		let mut sol = Solution::length_of_last_word("        fly me	t the 	moo ".to_string(),);
		assert_eq!(ans, sol);
	}
}

fn main() {}
