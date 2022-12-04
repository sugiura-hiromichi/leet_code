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

impl PartialOrd for ListNode {
	fn partial_cmp(&self, other: &Self,) -> Option<std::cmp::Ordering,> {
		self.val.partial_cmp(&other.val,)
	}
}
impl Ord for ListNode {
	fn cmp(&self, other: &Self,) -> std::cmp::Ordering { self.val.cmp(&other.val,) }
}

use std::cmp::Reverse;
impl Solution {
	pub fn merge_k_lists(lists: Vec<Option<Box<ListNode,>,>,>,) -> Option<Box<ListNode,>,> {
		let mut min_heap = std::collections::BinaryHeap::new();
		for l in lists {
			if let Some(nod,) = l {
				min_heap.push(Reverse(nod,),);
			}
		}

		let mut head = ListNode::new(0,);
		let mut cur = &mut head;
		while let Some(Reverse(nod,),) = min_heap.pop() {
			cur.next = Some(Box::new(ListNode::new(nod.val,),),);
			cur = cur.next.as_mut().unwrap();
			if let Some(nxt,) = nod.next {
				min_heap.push(Reverse(nxt,),);
			}
		}

		head.next
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = ary_to_list(&[1, 1, 2, 3, 4, 4, 5, 6,],);
		let mut sol =
			Solution::merge_k_lists(arys_to_lists(vec![&[1, 4, 5,], &[1, 3, 4,], &[2, 6,]],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![&[]],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = ary_to_list(&[1, 2, 3,],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![&[1,], &[2, 3,]],),);
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
