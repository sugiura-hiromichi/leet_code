#![allow(unused)]

struct Solution;
impl Solution {
	// d: It's time to study bit-manipulation
	pub fn is_valid_sudoku(board: Vec<Vec<char,>,>,) -> bool {
		let mut seen = std::collections::HashSet::new();
		for i in 0..9 {
			for j in 0..9 {
				let number = board[i][j];
				if number != '.' {
					if !seen.insert(format!("{number} in row {i}"),)
						|| !seen.insert(format!("{number} in column {j}"),)
						|| !seen.insert(format!("{number} in block {} - {}", i / 3, j / 3),)
					{
						return false;
					}
				}
			}
		}
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = true;
		let mut sol = Solution::is_valid_sudoku(vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = false;
		let mut sol = Solution::is_valid_sudoku(vec![
			vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = false;
		let mut sol = Solution::is_valid_sudoku(vec![
			vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
			vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
			vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
			vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
			vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
			vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
			vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
			vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
			vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
		],);
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

#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
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
	println!("2 is {} as u8", '2' as u8);
}
