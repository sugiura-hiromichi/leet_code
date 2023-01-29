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
	pub fn unique_paths(m: i32, n: i32,) -> i32 {
		let cargos = m + n - 2;
		let mut min = m.min(n,) - 1;
		if min == 0 {
			return 1;
		}
		let mut rslt: i64 = cargos as i64;
		for i in 1..min {
			rslt *= (cargos - i) as i64;
			rslt /= (i + 1) as i64;
		}
		rslt as i32
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
		let mut ans = 28;
		let mut sol = Solution::unique_paths(3, 7,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 3;
		let mut sol = Solution::unique_paths(3, 2,);
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
