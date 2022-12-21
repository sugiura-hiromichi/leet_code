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
	pub fn next_permutation(nums: &mut Vec<i32,>,) {
		for i in (1..nums.len()).rev() {
			if nums[i - 1] < nums[i] {
				let swap_pos = i + lexcal_nxt(nums[i..].to_vec(), nums[i - 1],);
				nums.swap(i - 1, swap_pos,);
				nums.get_mut(i..,).unwrap().sort();
				return;
			}
		}
		nums.sort();
	}
}

///# NOTE:
///  `lexcal_nxt` returns position of lexicographically next value
fn lexcal_nxt(v: Vec<i32,>, target: i32,) -> usize {
	/* more general algorhythm
	 let mut ret = (0, i32::MAX,);
	 v.iter().enumerate().for_each(|(pos, &i,)| {
		 if target < i && i < ret.1 {
			 ret = (pos, i,)
		 }
	 },);

	 ret.0
	*/
	// this assumes v is sorted reversely. use less memory
	v.iter().enumerate().filter(|&(pos, &i,)| i > target,).last().unwrap().0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![1, 3, 2];
		let mut sol = vec![1, 2, 3];
		Solution::next_permutation(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![1, 2, 3];
		let mut sol = vec![3, 2, 1];
		Solution::next_permutation(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![1, 5, 1];
		let mut sol = vec![1, 1, 5];
		Solution::next_permutation(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = vec![2, 1, 3];
		let mut sol = vec![1, 3, 2];
		Solution::next_permutation(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = vec![3, 1, 2];
		let mut sol = vec![2, 3, 1];
		Solution::next_permutation(&mut sol,);
		assert_eq!(ans, sol);
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
