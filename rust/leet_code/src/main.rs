#![allow(unused, dead_code)]
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
	pub fn swap_pairs(mut head: Option<Box<ListNode,>,>,) -> Option<Box<ListNode,>,> {
		let next_head = match head.clone() {
			Some(n,) if n.next.is_some() => n.clone().next.unwrap().next,
			_ => return head,
		};

		let tmp = head.clone();
		head = head.as_mut().unwrap().next.as_ref().cloned();
		head.as_mut().unwrap().next = tmp;
		head.as_mut().unwrap().next.as_mut().unwrap().next = Solution::swap_pairs(next_head,);
		head
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = ary_to_list(&[1, 1, 2, 3, 4, 4, 5, 6,],);
		let mut sol = Solution::swap_pairs(ary_to_list(&[1, 1, 3, 2, 4, 4, 6, 5,],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[2, 1, 4, 3,],);
		let mut sol = Solution::swap_pairs(ary_to_list(&[1, 2, 3, 4,],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::swap_pairs(ary_to_list(&[],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = ary_to_list(&[1,],);
		let mut sol = Solution::swap_pairs(ary_to_list(&[1,],),);
		assert_eq!(ans, sol);
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
}

fn main() {}
