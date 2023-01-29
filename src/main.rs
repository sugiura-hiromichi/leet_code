// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use std::rc::Rc;

use test::black_box;
use test::Bencher;

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

struct Solution;
impl Solution {
	pub fn rotate_right(mut head: Option<Box<ListNode,>,>, k: i32,) -> Option<Box<ListNode,>,> {
		let mut counter = &head;
		let mut len = 0;
		while counter.is_some() {
			len += 1;
			counter = &counter.as_ref().unwrap().next;
		}
		if len < 2 {
			return head;
		}
		// unmutalize
		let len = len;

		let k = k as usize % len;
		if k == 0 {
			return head;
		}

		let mut tmp = &mut head;
		for _i in 1..len - k {
			tmp = &mut tmp.as_mut().unwrap().next;
		}

		let mut new_head = tmp.clone().unwrap().next;
		tmp.as_mut().unwrap().next = None;
		let mut old_end = &mut new_head;
		for _i in 1..k {
			old_end = &mut old_end.as_mut().unwrap().next;
		}

		old_end.as_mut().unwrap().next = head;

		new_head.clone()
	}
}

fn ary_to_list(a: &[i32],) -> Option<Box<ListNode,>,> {
	Some(Box::new(ListNode {
		val:  a[0],
		next: if a.len() == 1 { None } else { ary_to_list(&a[1..],) },
	},),)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = ary_to_list(&[4, 5, 1, 2, 3,],);
		let mut sol = Solution::rotate_right(ary_to_list(&[1, 2, 3, 4, 5,],), 2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[2, 0, 1,],);
		let mut sol = Solution::rotate_right(ary_to_list(&[0, 1, 2,],), 4,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn list_next() {
		let mut l = &mut ary_to_list(&[1, 2, 3, 4,],);
		l = &mut l.as_mut().unwrap().next;
		assert_eq!(l.clone(), ary_to_list(&[2, 3, 4]));
	}
}

mod benchs {
	use super::*;

	//	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			// fastest vector init
			let mut v = vec![0; 1e5 as usize];
			for i in 0..1e5 as i32 {
				v[i as usize] = i;
			}
		},)
	}
}

fn main() { assert_eq!(21 - 15 % 6, 18) }
