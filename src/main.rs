#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn total_n_queens(n: i32,) -> i32 {
		let mut rslt = 0;
		Self::dfs(n as usize, 0, 0, 0, &mut rslt,);
		rslt
	}

	fn dfs(n: usize, diag_135: i32, diag_45: i32, col_mask: i32, rslt: &mut i32,) {
		// set 0~n-1th bit flags by `(1<<n)-1`
		if (1 << n) - 1 == col_mask {
			*rslt += 1;
			return;
		}
		let available = !(diag_135 | diag_45 | col_mask);

		for i in 0..n {
			let bit_inf = 1 << i;
			if available & bit_inf == 0 {
				continue;
			}

			Self::dfs(
				n,
				(diag_135 | bit_inf) >> 1,
				(diag_45 | bit_inf) << 1,
				col_mask | bit_inf,
				rslt,
			);
		}
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 2;
		let mut sol = Solution::total_n_queens(4,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 1;
		let mut sol = Solution::total_n_queens(1,);
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
