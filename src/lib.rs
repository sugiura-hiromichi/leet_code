#![allow(dead_code)]

type List = Option<Box<ListNode,>,>;

#[derive(Debug, PartialEq, Eq, Clone,)]
pub struct ListNode {
	pub val:  i32,
	pub next: List,
}
impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { val, next: None, } }
}

struct Solution;
impl Solution {
	pub fn partition(mut head: List, x: i32,) -> List {
		let (mut before, mut after,) = (ListNode::new(0,), ListNode::new(0,),);
		let (mut before_tail, mut after_tail,) = (&mut before, &mut after,);

		while let Some(mut node,) = head {
			// this `take()` is required to leave `None` at `node.next` because
			// node is used later
			head = node.next.take();

			if node.val < x {
				before_tail.next = Some(node,);
				before_tail = before_tail.next.as_mut().unwrap();
			} else {
				after_tail.next = Some(node,);
				after_tail = after_tail.next.as_mut().unwrap();
			}
		}

		before_tail.next = after.next;
		before.next
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_list(ary: &[i32],) -> List {
		if ary.len() == 0 {
			None
		} else {
			Some(Box::new(ListNode {
				val:  ary[0],
				next: create_list(&ary[1..],),
			},),)
		}
	}

	#[test]
	fn test_1() {
		let ans = create_list(&[1, 2, 2, 4, 3, 5,],);
		let sol = Solution::partition(create_list(&[1, 4, 3, 2, 5, 2,],), 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = create_list(&[1, 2,],);
		let sol = Solution::partition(create_list(&[2, 1,],), 2,);
		assert_eq!(ans, sol);
	}
}
