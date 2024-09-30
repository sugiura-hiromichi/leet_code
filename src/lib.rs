#![allow(dead_code)]


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
type List = Option<Box<ListNode,>,>;
impl Solution {
	/// # Parameters
	///
	/// - `rows == matrix.length`
	/// - `cols == matrix.length`
	/// - `1 <= row` `cols <= 200`
	/// - `matrix[i][j] = '0' | '1'`
	pub fn partition(head: List, x: i32,) -> List {
		let (mut before, mut after,) = (None, None,);
		let (mut before_tail, mut after_tail,) = (&mut before, &mut after,);
		let mut p_head = &head;

		while let Some(node,) = p_head {
			if node.val < x {
				*before_tail = Some(Box::new(ListNode::new(node.val,),),);
				before_tail = &mut before_tail.as_mut().unwrap().next;
			} else {
				*after_tail = Some(Box::new(ListNode::new(node.val,),),);
				after_tail = &mut after_tail.as_mut().unwrap().next;
			}
			p_head = &p_head.as_ref().unwrap().next;
		}

		*before_tail = after;

		before
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
