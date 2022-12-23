#![allow(unused)]

struct Solution;
impl Solution {
	pub fn longest_valid_parentheses(s: String,) -> i32 {
		let mut ret = 0;
		let mut head = 0;

		while let Some(i,) = s[head..].find('(',) {
			let mut valid = 0;
			let mut longest = 0;
			for (j, c,) in s[head + i..].chars().enumerate() {
				if c == '(' {
					valid += 1;
				} else {
					if 0 == valid {
						break;
					}
					valid -= 1;
				}

				if 0 == valid {
					longest = longest.max(j + 1,);
				}
			}

			ret = ret.max(longest,);
			if head + i + longest + 1 < s.len() {
				head += i + longest + 1;
			} else {
				break;
			}
		}

		ret as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// FAIL: `sol=2`
	#[test]
	fn test_1() {
		let mut ans = 6;
		let mut sol = Solution::longest_valid_parentheses("()((())()".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = 4;
		let mut sol = Solution::longest_valid_parentheses("(())".to_string(),);
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
