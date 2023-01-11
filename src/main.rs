#![allow(unused)]

struct Solution;
impl Solution {
	pub fn group_anagrams(mut strs: Vec<String,>,) -> Vec<Vec<String,>,> {
		let mut ret = vec![];
		while let Some(mut org,) = strs.pop() {
			let mut tmp = vec![org.clone()];
			let mut i = 0;
			while i < strs.len() {
				let mut str = strs[i].clone();
				if org.len() != str.len() {
					i += 1;
					continue;
				}
				//let str_len = str.len();
				org.chars().for_each(|c| {
					if let Some(i,) = str.find(c,) {
						str.remove(i,);
					}
				},);

				if str.is_empty() {
					tmp.push(strs.remove(i,),);
				} else {
					i += 1;
				}
			}
			ret.push(tmp,);
		}

		ret
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
		let mut sol =
			Solution::group_anagrams(vectorize(&[&["eat", "tea", "tan", "ate", "nat", "bat",],],)[0].clone(),);
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
