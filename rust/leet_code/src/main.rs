#![allow(unused, dead_code)]

struct Solution;
impl Solution {
	pub fn generate_parenthesis(n: i32,) -> Vec<String,> { back_track("".to_string(), n, 0,) }
}

fn back_track(s: String, open: i32, close: i32,) -> Vec<String,> {
	if open == 0 && close == 0 {
		return vec![s];
	}

	let mut ret = vec![];
	if open > 0 {
		ret.append(&mut back_track(s.clone() + "(", open - 1, close + 1,),);
	}
	if close > 0 {
		ret.append(&mut back_track(s.clone() + ")", open, close - 1,),);
	}
	ret
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
		let mut sol = Solution::generate_parenthesis(3,);

		ans.sort();
		sol.sort();

		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec!["()"];
		let mut sol = Solution::generate_parenthesis(1,);

		ans.sort();
		sol.sort();

		assert_eq!(ans, sol);
	}
}
