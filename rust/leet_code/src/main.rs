#![allow(unused)]

struct Solution;
impl Solution {
	pub fn solve_sudoku(board: &mut Vec<Vec<char,>,>,) -> bool {
		// NOTE: refactor to use bit manipulation later
		while let Some(t,) = has_dot(board,) {
			let mut validated = self_valid(t, board,);
			if validated == 0 {
				return false;
			}

			let mut th = 0;
			while validated != 0 {
				th += 1;
				validated = validated >> 1;
				if validated & 1 == 1 {
					board[t.0][t.1] = char::from_digit(th, 10,).unwrap();
					let mut b_clone = board.clone();
					if Self::solve_sudoku(&mut b_clone,) {
						*board = b_clone;
						return true;
					}
				}
			}
			// d: if program reach here, all valid number were invalid
			return false;
		}
		true
	}
}

/// returns `(yoko, tate)`
fn has_dot(board: &Vec<Vec<char,>,>,) -> Option<(usize, usize,),> {
	for i in 0..9 {
		for j in 0..9 {
			if board[i][j] == '.' {
				return Some((i, j,),);
			}
		}
	}
	None
}

/// returns valid flag to the position of `t`
/// a: take care that flag is set **1~9th**. **NOT 0~8th!**.
fn self_valid(t: (usize, usize,), board: &Vec<Vec<char,>,>,) -> u16 {
	yoko_valid(t, board,) & tate_valid(t, board,) & block_valid(t, board,)
}

fn yoko_valid(t: (usize, usize,), board: &Vec<Vec<char,>,>,) -> u16 {
	let mut is_valid = 0b1111111110u16;
	for i in 0..3 {
		for j in 0..3 {
			if let Some(c,) = board[t.0][i * 3 + j].to_digit(10,) {
				is_valid &= !(1 << c);
			}
		}
	}
	is_valid
}

fn tate_valid(t: (usize, usize,), board: &Vec<Vec<char,>,>,) -> u16 {
	let mut is_valid = 0b1111111110u16;
	for i in 0..3 {
		for j in 0..3 {
			if let Some(c,) = board[i * 3 + j][t.1].to_digit(10,) {
				is_valid &= !(1 << c);
			}
		}
	}
	is_valid
}

fn block_valid(t: (usize, usize,), board: &Vec<Vec<char,>,>,) -> u16 {
	let mut is_valid = 0b1111111110u16;
	for i in 0..3 {
		for j in 0..3 {
			if let Some(c,) = board[t.0 / 3 * 3 + i][t.1 / 3 * 3 + j].to_digit(10,) {
				is_valid &= !(1 << c);
			}
		}
	}
	is_valid
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![
			vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
			vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
			vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
			vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
			vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
			vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
			vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
			vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
			vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
		];
		let mut sol = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		Solution::solve_sudoku(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn arithmetic() {
		assert_eq!(2, 3 / 2 * 2);
		let mut is_valid = [0u16; 1];
		assert_eq!(is_valid, [0]);
	}

	#[test]
	fn test_self_valid() {
		let t = (0, 2,);
		let mut board = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		let sol = self_valid(t, &board,);
		let ans = 0b0000010110;
		assert_eq!(sol, ans);
	}

	#[test]
	fn test_yoko_valid() {
		let t = (0, 2,);
		let mut board = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		let sol = yoko_valid(t, &board,);
		let ans = 0b1101010110;
		assert_eq!(sol, ans);
	}

	#[test]
	fn test_tate_valid() {
		let t = (0, 2,);
		let mut board = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		let sol = tate_valid(t, &board,);
		let ans = 0b1011111110;
		assert_eq!(sol, ans);
	}

	#[test]
	fn test_block_valid() {
		let t = (0, 2,);
		let mut board = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		let sol = block_valid(t, &board,);
		let ans = 0b0010010110;
		assert_eq!(sol, ans);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
