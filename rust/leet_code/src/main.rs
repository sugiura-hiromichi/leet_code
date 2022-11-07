#![allow(dead_code)]
struct Solution;
impl Solution {
	pub fn letter_combinations(digits: String,) -> Vec<String,> {
		let push_list = std::collections::HashMap::from([
			('2', "abc",),
			('3', "def",),
			('4', "ghi",),
			('5', "jkl",),
			('6', "mno",),
			('7', "pqrs",),
			('8', "tuv",),
			('9', "wxyz",),
		],);

		let mut combs: Vec<String,> = vec!["".to_string()];
		for dig in digits.chars() {
			combs = push_list
				.get(&dig,)
				.unwrap()
				.chars()
				.flat_map(|c| {
					combs.iter().map(move |s| {
						let mut s = s.clone();
						s.push(c,);
						s
					},)
					//		.collect::<Vec<String,>>()
				},)
				.collect::<Vec<String,>>();
		}

		if combs == vec!["".to_string()] {
			vec![]
		} else {
			combs
		}
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut sol = Solution::letter_combinations("23".to_string(),);
		sol.sort();
		assert_eq!(sol, ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
	}

	#[test]
	fn test_2() {
		let empty: Vec<String,> = vec![];
		let mut sol = Solution::letter_combinations("".to_string(),);
		sol.sort();
		assert_eq!(sol, empty);
	}

	#[test]
	fn test_3() {
		let mut sol = Solution::letter_combinations("2".to_string(),);
		sol.sort();
		assert_eq!(sol, ["a", "b", "c"]);
	}
}
