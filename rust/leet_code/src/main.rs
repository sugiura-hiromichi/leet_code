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
	pub fn remove_duplicates(nums: &mut Vec<i32,>,) -> i32 {
		let mut i = 0;
		while let Some(x,) = nums.get(i,) {
			match nums.get(i + 1,) {
				Some(y,) => {
					if x == y {
						nums.remove(i,);
					} else {
						i += 1;
					}
				},
				None => break,
			}
		}

		1 + i as i32
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
		let nums = &mut vec![1, 1, 2];
		let mut ans = 2;
		let mut sol = Solution::remove_duplicates(nums,);
		assert_eq!(ans, sol);
		assert_eq!(nums, &[1, 2]);
	}

	#[test]
	fn test_2() {
		let nums = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
		let mut ans = 5;
		let mut sol = Solution::remove_duplicates(nums,);
		assert_eq!(ans, sol);
		assert_eq!(nums, &[0, 1, 2, 3, 4]);
	}
}

// use only when stdin is needed
fn main() {
	// todo-comments
	// FIX:
	// e:
	// TODO:
	// q:
	// HACK:
	// a:
	// WARN:
	// x:
	// PERF:
	// p:
	// NOTE:
	// d:
	// TEST:
	// t:
	// PASS:
	// FAIL:
}
