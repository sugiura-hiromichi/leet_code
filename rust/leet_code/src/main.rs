#![allow(unused)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
}

impl Solution {
	pub fn find_substring(s: String, mut words: Vec<String,>,) -> Vec<i32,> {
		let mut ret = vec![];
		// deal with edge cases
		if words.is_empty() {
			return ret;
		}

		// prepare
		words.sort();
		let word_len = words[0].len();
		let len = words.len() * word_len;

		let mut word_list = words.clone();
		word_list.dedup();
		for word in word_list {
			let mut pad = 0;

			// find candidate's head
			while let Some(i,) = s[pad..].find(&word,) {
				let mut candidate = match s.get(i + pad..i + pad + len,) {
					// iterator over each words
					Some(cand,) => cand
						.char_indices()
						.filter_map(|(j, _,)| {
							if (j + 1) % word_len == 0 {
								Some(cand[j + 1 - word_len..=j].to_string(),)
							} else {
								None
							}
						},)
						.collect::<Vec<String,>>(),
					None => break,
				};
				candidate.sort();

				if words == candidate {
					ret.push((pad + i) as i32,);
				}

				// prepare for next loop
				pad += i + 1;
			}
		}
		ret
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![0, 9];
		let mut sol = Solution::find_substring(
			"barfoothefoobarman".to_string(),
			vec!["foo".to_string(), "bar".to_string()],
		);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans: Vec<i32,> = vec![];
		let mut sol = Solution::find_substring(
			"wordgoodgoodgoodbestword".to_string(),
			vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()],
		);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![6, 9, 12];
		let mut sol = Solution::find_substring(
			"barfoofoobarthefoobarman".to_string(),
			vec!["bar".to_string(), "foo".to_string(), "the".to_string()],
		);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = vec![8];
		let mut sol = Solution::find_substring(
			"wordgoodgoodgoodbestword".to_string(),
			vec!["word".to_string(), "good".to_string(), "best".to_string(), "good".to_string()],
		);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let mut sol = Solution::find_substring(
			"aaaaaaaaaaaaaa".to_string(),
			vec!["aa".to_string(), "aa".to_string()],
		);
		sol.sort();
		assert_eq!(ans, sol);
	}
}

fn ary_to_list(ary: &[i32],) -> Option<Box<ListNode,>,> {
	if ary.is_empty() {
		None
	} else {
		Some(Box::new(ListNode { val: ary[0], next: ary_to_list(&ary[1..],), },),)
	}
}

fn arys_to_lists(arys: Vec<&[i32],>,) -> Vec<Option<Box<ListNode,>,>,> {
	arys.iter().map(|&a| ary_to_list(a,),).collect()
}

// use only when stdin is needed
fn main() {
	// todo-comments
	// FIX:
	// e: `e` stands for "error"
	// TODO:
	// q: `q` stands for "question"
	// HACK:
	// a: `a` stands for "attention"
	// WARN:
	// x: `x` is abbreviation of "XXX"
	// PERF:
	// p: `p` stands for "performance"
	// NOTE:
	// d: `d` stands for "description"
	// TEST:
	// t: `t` stands for "test"
	// PASS:
	// FAIL:
}
