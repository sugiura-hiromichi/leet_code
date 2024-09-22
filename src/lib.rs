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
		let last_val =
			if let Some(node,) = &head { node.val - 1 } else { return head };
		Self::builder(head, last_val,)
	}

	fn builder(
		list: Option<Box<ListNode,>,>,
		last_val: i32,
	) -> Option<Box<ListNode,>,> {
		match list {
			Some(node,) => {
				if node.val != last_val
					&& ((node.next.is_some()
						&& node.val != node.next.as_ref().unwrap().val)
						|| node.next.is_none())
				{
					Some(Box::new(ListNode {
						val:  node.val,
						next: Self::builder(node.next, node.val,),
					},),)
				} else {
					Self::builder(node.next, node.val,)
				}
			},
			None => None,
		}
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
