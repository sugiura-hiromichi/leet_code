#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn solve_n_queens(n: i32,) -> Vec<Vec<String,>,> {
		let mut rslt = vec![];
		Self::dfs(n as usize, 0, 0, 0, &mut rslt, &mut vec![],);
		rslt
	}

	fn dfs(
		n: usize,
		diag_135: i32,
		diag_45: i32,
		col_mask: i32,
		rslt: &mut Vec<Vec<String,>,>,
		path: &mut Vec<usize,>,
	) {
		let bitmask = (1 << n) - 1;
		if bitmask == col_mask {
			rslt.push(Self::decode(path, n,),);
			return;
		}
		let available = bitmask & (!(diag_135 | diag_45 | col_mask));

		for i in 0..n {
			let bit_inf = 1 << i;
			if available & bit_inf == 0 {
				continue;
			}
			path.push(i,);

			Self::dfs(
				n,
				(diag_135 | bit_inf) >> 1,
				(diag_45 | bit_inf) << 1,
				col_mask | bit_inf,
				rslt,
				path,
			);
			path.pop();
		}
	}

	fn decode(path: &Vec<usize,>, n: usize,) -> Vec<String,> {
		path.iter()
			.enumerate()
			.fold(vec![vec!['.'; n]; n], |mut acc, (i, &j,)| {
				acc[i][j] = 'Q';
				acc
			},)
			.iter()
			.map(|c| c.iter().collect(),)
			.collect()
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans =
			vec![vec![".Q..", "...Q", "Q...", "..Q."], vec!["..Q.", "Q...", "...Q", ".Q.."]];
		let mut sol = Solution::solve_n_queens(4,);
		sol.sort();
		ans.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec!["Q"]];
		let mut sol = Solution::solve_n_queens(1,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn label() {
		let mut v = vec![];
		'label: for i in 0..10 {
			if i % 2 == 0 {
				continue 'label;
			}
			v.push(i,);
		}

		assert_eq!(v, vec![1, 3, 5, 7, 9])
	}
}

fn main() {}
