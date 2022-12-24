#![allow(unused)]

struct Solution;
impl Solution {
	pub fn longest_valid_parentheses(s: String,) -> i32 {
		if s.len() <= 1 {
			return 0;
		}
		let mut longest = vec![0; s.len()];

		for i in 1..s.len() {
			if &s[i..=i] == ")"
				&& i - longest[i - 1] > 0
				&& &s[i - longest[i - 1] - 1..i - longest[i - 1]] == "("
			{
				longest[i] = longest[i - 1] + 2;
				if i - longest[i - 1] > 1 {
					longest[i] += longest[i - longest[i - 1] - 2];
				}
			}
		}

		*longest.iter().max().unwrap() as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// FAIL: `sol=4`
	#[test]
	fn test_1() {
		let mut ans = 6;
		let mut sol = Solution::longest_valid_parentheses("()((())()".to_string(),);
		assert_eq!(ans, sol);
	}

	// FAIL: `sol=2`
	#[test]
	fn test_2() {
		let mut ans = 4;
		let mut sol = Solution::longest_valid_parentheses(")(()((((())".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 4;
		let mut sol = Solution::longest_valid_parentheses(")()())()()(".to_string(),);
		assert_eq!(ans, sol);
	}

	// FAIL: `sol=2`
	#[test]
	fn test_4() {
		let mut ans = 4;
		let mut sol = Solution::longest_valid_parentheses("(())".to_string(),);
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

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
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
