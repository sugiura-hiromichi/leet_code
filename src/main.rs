#![allow(unused)]

const EPSILON: f64 = 1e-10;

struct Solution;
impl Solution {
	pub fn solve_n_queens(n: i32,) -> Vec<Vec<String,>,> {
		let n = n as usize;
		let mut v = vec![];
		for i in 0..n {
			v.push(i,);
		}
		let mut list = Self::permute(v,);

		let mut rslt = vec![];
		'list: for b in list {
			// validate angled line
			for i in 0..n {
				// 135˚
				let (mut col, mut row,) = if b[i] < i { (0, i - b[i],) } else { (b[i] - i, 0,) };
				while col < n && row < n {
					if !(i == row) && b[row] == col {
						continue 'list;
					}
					row += 1;
					col += 1;
				}

				// 45˚
				let (mut ano_col, mut ano_row,) = if i + b[i] > n - 1 {
					(0, i + b[i] - n + 1,)
				} else {
					(n - 1 - (i + b[i]), 0,)
				};
				while ano_col < n && ano_row < n {
					if !(i == ano_row) && b[ano_row] == n - 1 - ano_col {
						continue 'list;
					}
					ano_row += 1;
					ano_col += 1;
				}
			}

			let mut tmp = vec![];
			b.iter().for_each(|i| tmp.push(".".repeat(*i,) + "Q" + &".".repeat(n - i - 1,),),);
			rslt.push(tmp,);
		}

		rslt
	}

	fn permute(nums: Vec<usize,>,) -> Vec<Vec<usize,>,> {
		let len = nums.len();
		if len == 1 {
			return vec![nums];
		}
		let mut ret = vec![];
		for i in 0..len {
			let mut cl = nums.clone();
			let rm = cl.remove(i,);
			Self::permute(cl,).iter_mut().for_each(|v| {
				v.push(rm,);
				ret.push(v.clone(),);
			},)
		}

		ret
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
