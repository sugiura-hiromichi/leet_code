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

trait AltAbs {
	fn minus_abs(self,) -> Self;
}

impl AltAbs for (i32, i32,) {
	fn minus_abs(self,) -> Self {
		let a = if self.0 > 0 { -self.0 } else { self.0 };
		let b = if self.1 > 0 { -self.1 } else { self.1 };
		(a, b,)
	}
}

impl Solution {
	pub fn divide(dividend: i32, divisor: i32,) -> i32 {
		if divisor == -1 {
			if dividend == i32::MIN {
				return i32::MAX;
			} else {
				return -dividend;
			}
		} else if divisor == 1 {
			return dividend;
		}
		if dividend == 0 {
			return 0;
		}

		// d: dd & ds are minus
		let (dd, ds,) = (dividend, divisor,).minus_abs();
		let mut no_frac = dd;

		// q: sub from dd and check if dd<0
		let mut subs = vec![(ds, 1,)];
		let mut tmp = (ds, 1,);
		while !tmp.0.overflowing_add(tmp.0,).1 && no_frac < tmp.0 {
			tmp = (tmp.0 + tmp.0, tmp.1 + tmp.1,);
			subs.push(tmp,);
		}

		let mut quo = 0;
		for (sub, qs,) in subs.iter().rev() {
			while no_frac <= *sub {
				no_frac -= sub;
				quo += qs;
			}
		}

		if dividend > 0 && divisor > 0 || dividend < 0 && divisor < 0 {
			quo
		} else {
			-quo
		}
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
		let mut ans = 3;
		let mut sol = Solution::divide(10, 3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = -2;
		let mut sol = Solution::divide(7, -3,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 2147483647;
		let mut sol = Solution::divide(2147483647, 1,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = 1073741823;
		let mut sol = Solution::divide(2147483647, 2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = 1;
		let mut sol = Solution::divide(2, 2,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_6() {
		let mut ans = -1073741824;
		let mut sol = Solution::divide(-2147483648, 2,);
		assert_eq!(ans, sol);
	}
}

// use only when stdin is needed
fn main() {
	// todo-comments
	// FIX:
	// e: `e` stands for "error"
	// TODO:
	// q: `q` stands for "question"
	// HACK:
	// a: `a` stands for "attention"
	// WARN:
	// x: `x` is abbreviation of "XXX"
	// PERF:
	// p: `p` stands for "performance"
	// NOTE:
	// d: `d` stands for "description"
	// TEST:
	// t: `t` stands for "test"
	// PASS:
	// FAIL:
}
