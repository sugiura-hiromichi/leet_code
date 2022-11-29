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

impl Iterator for ListNode {
	type Item = Box<ListNode,>;

	fn next(&mut self,) -> Option<Self::Item,> { self.clone().next }
}

impl Solution {
	pub fn remove_nth_from_end(
		mut head: Option<Box<ListNode,>,>,
		n: i32,
	) -> Option<Box<ListNode,>,> {
		let mut pos = 0;
		let mut target = head.as_ref();
		// Calculate position to remove
		while target.is_some() {
			pos += 1;
			target = target.unwrap().next.as_ref();
		}
		pos -= n;

		// NOTE: this `unwrap()` always success because first Node is always Some()
		list_to_list(Some(Box::new(ListNode { val: 0, next: head, },),), pos,).unwrap().next
	}
}

/// INFO: create list by recursion ignoring if n==0
fn list_to_list(l: Option<Box<ListNode,>,>, n: i32,) -> Option<Box<ListNode,>,> {
	match l {
		None => None,
		Some(nod,) => Some(Box::new(ListNode {
			val:  nod.val,
			next: list_to_list(if n == 0 { nod.next.unwrap().next } else { nod.next }, n - 1,),
		},),),
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	// NOTE: helper `fn` for test
	fn ary_to_list(ary: &[i32],) -> Option<Box<ListNode,>,> {
		if ary.len() == 0 {
			None
		} else {
			Some(Box::new(ListNode { val: ary[0], next: ary_to_list(&ary[1..],), },),)
		}
	}

	#[test]
	fn test_1() {
		let mut ans = ary_to_list(&[1, 2, 3, 5,],);
		let mut sol = Solution::remove_nth_from_end(ary_to_list(&[1, 2, 3, 4, 5,],), 2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::remove_nth_from_end(ary_to_list(&[1,],), 1,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = ary_to_list(&[1,],);
		let mut sol = Solution::remove_nth_from_end(ary_to_list(&[1, 2,],), 1,);
		assert_eq!(ans, sol);
	}
}
