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
	/// - list is guaranteed to be sorted in ascending order
	pub fn delete_duplicates(
		mut head: Option<Box<ListNode,>,>,
	) -> Option<Box<ListNode,>,> {
		// TODO: Consider using `Weak reference`
		let mut p_list = &mut head;

		while let Some(node,) = p_list {
			if node.next.is_some() {
				if node.val == node.next.as_ref().unwrap().val {
					node.next = node.next.as_mut().unwrap().next.take();
				} else {
					p_list = &mut p_list.as_mut().unwrap().next;
				}
			} else {
				break;
			}
		}

		head
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
		let ans = create_list(&[1, 2,],);
		let sol = Solution::delete_duplicates(create_list(&[1, 1, 2,],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = create_list(&[1, 2, 3,],);
		let sol = Solution::delete_duplicates(create_list(&[1, 1, 2, 3, 3,],),);
		assert_eq!(ans, sol);
	}
}
