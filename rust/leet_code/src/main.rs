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
	pub fn remove_element(nums: &mut Vec<i32,>, val: i32,) -> i32 {
		nums.retain(|&x| x != val,);
		nums.len() as i32
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let nums = &mut vec![3, 2, 2, 3];
		let mut ans = 2;
		let mut sol = Solution::remove_element(nums, 3,);
		assert_eq!(ans, sol);
		assert_eq!(nums, &[2, 2]);
	}

	#[test]
	fn test_2() {
		let nums = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
		let mut ans = 5;
		let mut sol = Solution::remove_element(nums, 2,);
		assert_eq!(ans, sol);
		assert_eq!(nums, &[0, 1, 3, 0, 4]);
	}
}

// use only when stdin is needed
fn main() {
	// todo-comments
	// FIX:
	// e:
	// TODO:
	// q:
	// HACK:
	// a:
	// WARN:
	// x:
	// PERF:
	// p:
	// NOTE:
	// d:
	// TEST:
	// t:
	// PASS:
	// FAIL:
}
