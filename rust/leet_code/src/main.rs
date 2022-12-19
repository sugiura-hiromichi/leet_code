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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
impl Solution {
	/// implement `find_substring` using `HashMap` and `hash_map::Entry`
	pub fn find_substring(s: String, mut words: Vec<String,>,) -> Vec<i32,> {
		let mut start_indices = Vec::<i32,>::new();
		if words.is_empty() {
			return start_indices;
		}

		let word_size = words[0].len();
		let len = word_size * words.len();

		if let Some(sub_case,) = s.len().checked_sub(len,) {
			// PERF: `::with_capacity()` is faster than `::new()`
			let mut word_set = HashMap::with_capacity(words.len(),);
			words.iter().for_each(|w| {
				// q: What is `.entry()` method?
				let count = word_set.entry(w.as_str(),).or_insert(0,);
				*count += 1;
			},);

			// subs constructed from `word_set`. this is required by `match` 13 lines below
			let mut subs = word_set.keys().map(|k| (*k, 0,),).collect::<HashMap<_, _,>>();
			// d: case like `s="aa"`, `words=["a","a"]` requires calling `.min()`
			for i in 0..word_size.min(sub_case + 1,) {
				// `i` stands for **initial**, `f` stands for **final**
				let mut f = i + len;
				while f <= s.len() {
					let mut k = 1;
					while k <= words.len() {
						let cur_pos = f - k * word_size;
						let cur = &s[cur_pos..cur_pos + word_size];

						//whether `subs` has key `cur` q: Search about `Entry` enum
						match subs.entry(cur,) {
							Entry::Occupied(ent,) => {
								let res = ent.into_mut();
								*res += 1;
								if *res > *word_set.get(cur,).unwrap() {
									// in this case, substrings contains `cur` overly
									break;
								} else {
									k += 1;
								}
							},
							Entry::Vacant(_,) => break,
						}
					}

					// adjust `f` and `subs`, and if substring found, push to start_indices.
					let start = f - len;
					if k > words.len() {
						start_indices.push(start as i32,);
						f += word_size;
					} else {
						f += len - (k - 1) * word_size;
					}
					subs.values_mut().for_each(|v| *v = 0,)
				}
			}
		}
		start_indices
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
