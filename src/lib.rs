#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
}

struct Solution;
impl Solution {
	/// # Parameters
	///
	/// - The number of nodes in the list is [0,300]
	/// - `-100<= Nodde.val <= 100`
	pub fn delete_duplicates(
		head: Option<Box<ListNode,>,>,
	) -> Option<Box<ListNode,>,> {
		let mut last_val = match &head {
			Some(l,) => l.val - 1,
			None => return None,
		};

		let mut rslt = ListNode::new(last_val,);
		let mut p_rslt = &mut rslt;
		let mut duplicated = false;

		let mut p_head = &head;

		while let Some(node,) = p_head {
			match (duplicated, last_val == node.val,) {
				(true, false,) => {
					duplicated = false;
					p_rslt.next = Some(Box::new(ListNode::new(node.val,),),);
					p_rslt = p_rslt.next.as_mut().unwrap();
				},
				(false, true,) => duplicated = true,
				// NOTE: consider the case (false,false). we need to update rslt
				// in the case
				(false, false,) => {
					p_rslt.next = Some(Box::new(ListNode::new(node.val,),),);
					p_rslt = p_rslt.next.as_mut().unwrap();
				},
				_ => (),
			}
			last_val = node.val;
			p_head = &node.next;
		}

		rslt.next
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_list(from: &[i32],) -> Option<Box<ListNode,>,> {
		if from.len() == 0 {
			None
		} else {
			Some(Box::new(ListNode {
				val:  from[0],
				next: create_list(&from[1..],),
			},),)
		}
	}

	#[test]
	fn test_1() {
		let ans = create_list(&[1, 2, 5,],);
		let sol =
			Solution::delete_duplicates(create_list(&[1, 2, 3, 3, 4, 4, 5,],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = create_list(&[2, 3,],);
		let sol = Solution::delete_duplicates(create_list(&[1, 1, 1, 2, 3,],),);
		assert_eq!(ans, sol);
	}
}
