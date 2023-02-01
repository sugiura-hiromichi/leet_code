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
	pub fn min_path_sum(mut map: Vec<Vec<i32,>,>,) -> i32 {
		let (row, col,) = (map.len(), map[0].len(),);
		for i in 1..row {
			map[i][0] += map[i - 1][0];
		}
		for i in 1..col {
			map[0][i] += map[0][i - 1];
		}

		for i in 1..row {
			for j in 1..col {
				map[i][j] += map[i - 1][j].min(map[i][j - 1],);
			}
		}
		map[row - 1][col - 1]
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
		let mut ans = 7;
		let mut sol = Solution::min_path_sum(vec![
			vec![1, 3, 1],
			vec![1, 5, 1],
			vec![4, 2, 1],
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 12;
		let mut sol = Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]],);
		assert_eq!(ans, sol);
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
