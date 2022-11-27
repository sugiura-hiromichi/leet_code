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
		let mut pos = 1;
		let mut target = head.as_ref();
		// Calculate position to remove
		while target.is_some() {
			pos += 1;
			target = target.unwrap().next.as_ref();
		}
		pos -= n;

		// Go to Nth node from end
		let mut target = head.as_mut();
		for _ in 0..pos {
			target = target.unwrap().next.as_mut();
		}

		// Remove
		let mut rm = target;
		rm = rm.unwrap().next.as_mut();

		head
	}
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 0;
		let mut sol = 0;
		assert_eq!(sol, ans);
	}
}
