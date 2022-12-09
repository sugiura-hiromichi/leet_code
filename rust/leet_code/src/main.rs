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
	pub fn reverse_k_group(mut head: Option<Box<ListNode,>,>, k: i32,) -> Option<Box<ListNode,>,> {
		let mut cur = head.clone();

		// find k+1th node
		let mut i = 0;
		while cur != None && i != k {
			cur = cur?.next;
			i += 1;
		}

		if i == k {
			// reverse rest k groups with k+1th node as head
			cur = Solution::reverse_k_group(cur, k,);

			// reverse current k group
			for _ in 0..i {
				let tmp = head.clone()?.next;
				head.as_mut()?.next = cur; // create reversed k+1-ith node
				cur = head.clone();
				head = tmp;
			}
			head = cur;
		}
		head
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
		let mut ans = ary_to_list(&[5, 4, 3, 2, 1,],);
		let mut sol = Solution::reverse_k_group(ary_to_list(&[1, 2, 3, 4, 5,],), 5,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[3, 2, 1, 4, 5,],);
		let mut sol = Solution::reverse_k_group(ary_to_list(&[1, 2, 3, 4, 5,],), 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = ary_to_list(&[3, 2, 1,],);
		let mut sol = Solution::reverse_k_group(ary_to_list(&[1, 2, 3,],), 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = ary_to_list(&[4, 3, 2, 1,],);
		let mut sol = Solution::reverse_k_group(ary_to_list(&[1, 2, 3, 4,],), 4,);
		assert_eq!(ans, sol);
	}
}

// use only when stdin is needed
fn main() {
	let mut ans = ary_to_list(&[5, 4, 3, 2, 1,],);
	let mut sol = Solution::reverse_k_group(ary_to_list(&[1, 2, 3, 4, 5,],), 5,);
	assert_eq!(ans, sol);
}
