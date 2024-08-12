#![allow(dead_code)]

struct Solution;
impl Solution {
	pub fn exist(board: Vec<Vec<char,>,>, word: String,) -> bool {
		let (row, col,) = (board.len(), board[0].len(),);
		let mut rslt = false;

		'l: for i in 0..row {
			for j in 0..col {
				let mut search_map =
					std::collections::HashSet::<(usize, usize,),>::new();
				search_map.insert((i, j,),);
				let mut w = word.chars();
				if board[i][j]
					== w.next().expect("`w` must contain more than 1 elements",)
					&& Self::char_match(search_map, w, &board, i, j,)
				{
					rslt = true;
					break 'l;
				}
			}
		}
		rslt
	}

	/// make sure the b[row][col] == w_val
	fn char_match(
		map: std::collections::HashSet<(usize, usize,),>,
		mut w: std::str::Chars,
		b: &Vec<Vec<char,>,>,
		row: usize,
		col: usize,
	) -> bool {
		if let Some(w_val,) = w.next() {
			let mut cord_list = vec![];

			if row != 0 {
				//↑
				cord_list.push((row - 1, col,),);
			}
			if row + 1 != b.len() {
				//↓
				cord_list.push((row + 1, col,),);
			}
			if col != 0 {
				//←
				cord_list.push((row, col - 1,),);
			}
			if col + 1 != b[0].len() {
				//→
				cord_list.push((row, col + 1,),);
			}

			cord_list.iter().any(|(r, c,)| {
				let mut m = map.clone();
				b[*r][*c] == w_val
					&& m.insert((*r, *c,),)
					&& Self::char_match(m, w.clone(), b, *r, *c,)
			},)
		} else {
			true
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let ans = true;
		let sol = Solution::exist(
			vec![
				vec!['A', 'B', 'C', 'E'],
				vec!['S', 'F', 'C', 'S'],
				vec!['A', 'D', 'E', 'E'],
			],
			"ABCCED".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = true;
		let sol = Solution::exist(
			vec![
				vec!['A', 'B', 'C', 'E'],
				vec!['S', 'F', 'C', 'S'],
				vec!['A', 'D', 'E', 'E'],
			],
			"SEE".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let ans = false;
		let sol = Solution::exist(
			vec![
				vec!['A', 'B', 'C', 'E'],
				vec!['S', 'F', 'C', 'S'],
				vec!['A', 'D', 'E', 'E'],
			],
			"ABCB".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let ans = false;
		let sol = Solution::exist(vec![vec!['a', 'a']], "aaa".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let ans = true;
		let sol = Solution::exist(
			vec![
				vec!['A', 'B', 'C', 'E'],
				vec!['S', 'F', 'E', 'S'],
				vec!['A', 'D', 'E', 'E'],
			],
			"ABCESEEEFS".to_string(),
		);
		assert_eq!(ans, sol);
	}
}
