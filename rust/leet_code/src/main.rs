#![allow(unused, dead_code)]

struct Solution;
impl Solution {
	pub fn generate_parenthesis(n: i32,) -> Vec<String,> {
		let mut hash = std::collections::HashSet::new();

		if n == 1 {
			vec![String::from("()",)]
		} else {
			let mut pre = Solution::generate_parenthesis(n - 1,);
			for mut s in pre {
				for i in 0..=s.len() {
					let mut s = s.clone();
					s.insert_str(i, "()",);
					hash.insert(s,);
				}
			}

			let mut ret = vec![];
			for s in hash.iter() {
				ret.push(s.clone(),);
			}

			ret
		}
	}
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
