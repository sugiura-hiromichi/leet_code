#![allow(unused)]

struct Solution;
impl Solution {
	pub fn search_range(nums: Vec<i32,>, target: i32,) -> Vec<i32,> {
		let i = nums.partition_point(|n| n < &target,);
		if i == nums.len() || nums[i] != target {
			return vec![-1, -1];
		}

		let j = nums.partition_point(|n| n <= &target,);
		vec![i as i32, j as i32 - 1]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![3, 4];
		let mut sol = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![-1, -1];
		let mut sol = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![-1, -1];
		let mut sol = Solution::search_range(vec![], 0,);
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
