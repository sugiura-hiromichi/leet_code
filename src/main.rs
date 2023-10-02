// •‿•
// lua vim.lsp.buf.inlay_hint(0)
#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn exist(board: Vec<Vec<char,>,>, word: String,) -> bool {
		enum Direction {
			Up,
			Down,
			Left,
			Right,
			None,
		}
		let mut candidates = vec![];
		let mut is_front = true;
		let (len, r_max, c_max,) = (word.len(), board.len(), board[0].len(),);
		let (front, back,) = (
			std::char::from_u32(word.as_bytes()[0] as u32,).unwrap(),
			std::char::from_u32(word.as_bytes()[len - 1] as u32,).unwrap(),
		);
		let (mut r, mut c,) = (0, 0,);

		for i in 0..r_max {
			for j in 0..c_max {
				if board[i][j] == front {
					Self::catch_up(&board, i, j,);
				} else if board[i][j] == back {
					Self::catch_up_rev(&board, i, j,);
				}
			}
		}

		todo!()
	}

	fn catch_up(board: &Vec<Vec<char,>,>, r: usize, c: usize,) -> bool { todo!() }

	fn catch_up_rev(board: &Vec<Vec<char,>,>, r: usize, c: usize,) -> bool { todo!() }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = true;
		let mut sol = Solution::exist(
			vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
			"ABCCED".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = true;
		let mut sol = Solution::exist(
			vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
			"SEE".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = false;
		let mut sol = Solution::exist(
			vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
			"ABCB".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn from_bool() {
		let mut ans = 1;
		let mut sol = usize::from(true,);
		assert_eq!(ans, sol);
		ans = 0;
		sol = usize::from(false,);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v.last(),);
			}
		},)
	}

	#[bench]
	fn b2(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v[v.len() - 1],);
			}
		},)
	}
}

fn main() {}
