#![allow(unused)]

struct Solution;
impl Solution {
	pub fn group_anagrams(mut strs: Vec<String,>,) -> Vec<Vec<String,>,> {
		if strs.len() == 0 {
			return vec![];
		}

		let mut ret: std::collections::HashMap<[i32; 26], Vec<String,>,> =
			std::collections::HashMap::new();
		for str in strs {
			let mut count = [0; 26];
			for c in str.chars() {
				count[c as usize - 'a' as usize] += 1;
			}

			match ret.get_mut(&count,) {
				Some(v,) => {
					v.push(str,);
				},
				None => {
					ret.insert(count, vec![str],);
				},
			}
		}

		ret.values().map(|v| v.clone(),).collect()
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	fn vectorize(ary: &[&[&str]],) -> Vec<Vec<String,>,> {
		ary.iter().map(|a| a.iter().map(|s| s.to_string(),).collect(),).collect()
	}

	#[test]
	fn test_1() {
		let mut ans = vectorize(&[&["bat",], &["nat", "tan",], &["ate", "eat", "tea",],],);
		let mut sol = Solution::group_anagrams(
			vectorize(&[&["eat", "tea", "tan", "ate", "nat", "bat",],],)[0].clone(),
		);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		ans.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vectorize(&[&["",],],);
		let mut sol = Solution::group_anagrams(vectorize(&[&["",],],)[0].clone(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vectorize(&[&["a",],],);
		let mut sol = Solution::group_anagrams(vectorize(&[&["a",],],)[0].clone(),);
		assert_eq!(ans, sol);
	}
}

fn main() {}
