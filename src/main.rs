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
	pub fn unique_paths_with_obstacles(map: Vec<Vec<i32,>,>,) -> i32 {
		if map[0][0] == 1 {
			return 0;
		}
		// TODO: dp
		let (row, col,) = (map.len(), map[0].len(),);
		let mut dp = vec![vec![0; col]; row];

		dp[0][0] = 1;
		for i in 1..col {
			dp[0][i] = i32::from(map[0][i] == 0 && dp[0][i - 1] == 1,);
		}
		for i in 1..row {
			dp[i][0] = i32::from(map[i][0] == 0 && dp[i - 1][0] == 1,);
		}

		for i in 1..row {
			for j in 1..col {
				if map[i][j] == 0 {
					dp[i][j] = if map[i - 1][j] == 0 { dp[i - 1][j] } else { 0 }
						+ if map[i][j - 1] == 0 { dp[i][j - 1] } else { 0 };
				}
			}
		}
		dp[row - 1][col - 1]
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
		let mut ans = 2;
		let mut sol = Solution::unique_paths_with_obstacles(vec![
			vec![0, 0, 0],
			vec![0, 1, 0],
			vec![0, 0, 0],
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 1;
		let mut sol = Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 1;
		let mut sol = Solution::unique_paths_with_obstacles(vec![vec![0]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = 2;
		let mut sol = Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0]],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn list_next() {
		assert_eq!(i32::from(true), 1);
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
