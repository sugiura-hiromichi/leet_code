#![allow(unused)]

struct Solution;
impl Solution {
	pub fn search_insert(nums: Vec<i32,>, target: i32,) -> i32 {
		nums.partition_point(|n| n < &target,) as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 2;
		let mut sol = Solution::search_insert(vec![1, 3, 5, 6], 5,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 1;
		let mut sol = Solution::search_insert(vec![1, 3, 5, 6], 2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 4;
		let mut sol = Solution::search_insert(vec![1, 3, 5, 6], 7,);
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
