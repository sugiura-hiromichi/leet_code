#![allow(unused, dead_code)]

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
}

struct Solution;
impl Solution {
	pub fn merge_two_lists(
		mut list1: Option<Box<ListNode,>,>,
		mut list2: Option<Box<ListNode,>,>,
	) -> Option<Box<ListNode,>,> {
		let mut merge = ListNode::new(0,);
		let mut p = &mut merge.next;
		loop {
			match (list1.clone(), list2.clone(),) {
				(None, None,) => break,
				(None, mut nxt,) | (mut nxt, None,) => {
					*p = nxt;
					break;
				},
				(Some(one,), Some(two,),) => {
					if one.val < two.val {
						*p = Some(Box::new(ListNode::new(one.val,),),);
						list1 = list1.unwrap().next;
					} else {
						*p = Some(Box::new(ListNode::new(two.val,),),);
						list2 = list2.unwrap().next;
					}

					p = &mut p.as_mut().unwrap().next;
				},
			}
		}

		merge.next
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	fn ary_to_list(ary: &[i32],) -> Option<Box<ListNode,>,> {
		if ary.len() == 0 {
			None
		} else {
			Some(Box::new(ListNode { val: ary[0], next: ary_to_list(&ary[1..],), },),)
		}
	}

	#[test]
	fn test_1() {
		let list1 = ary_to_list(&[1, 2, 4,],);
		let list2 = ary_to_list(&[1, 3, 4,],);
		let mut ans = ary_to_list(&[1, 1, 2, 3, 4, 4,],);
		let mut sol = Solution::merge_two_lists(list1, list2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let list1 = ary_to_list(&[],);
		let list2 = ary_to_list(&[],);
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::merge_two_lists(list1, list2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let list1 = ary_to_list(&[],);
		let list2 = ary_to_list(&[1,],);
		let mut ans = ary_to_list(&[1,],);
		let mut sol = Solution::merge_two_lists(list1, list2,);
		assert_eq!(ans, sol);
	}
}
