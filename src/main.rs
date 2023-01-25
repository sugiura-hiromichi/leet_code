#![allow(unused)]

const EPSILON: f64 = 1e-10;
struct Solution;

#[derive(Clone,)]
enum InsPos {
	Left(usize,),
	Between(usize,),
	Right,
}

impl InsPos {
	fn unwrap(self, len: usize,) -> usize {
		match self {
			Self::Right => len,
			Self::Between(n,) | Self::Left(n,) => n,
		}
	}
}

impl Solution {
	pub fn insert(itv: Vec<Vec<i32,>,>, new_itv: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let len = itv.len();
		if len == 0 {
			return vec![new_itv];
		}

		let mut rslt = vec![];
		let (start, end,) = (new_itv[0], new_itv[1],);
		let (mut s, mut e,) = (InsPos::Right, InsPos::Right,);

		for i in 1..itv.len() {
			if itv[i][0] <= start && start <= itv[i][1] {
				s = InsPos::Between(i,);
			} else if itv[i - 1][1] < start && start < itv[i][0] {
				s = InsPos::Left(i,);
			}
			if itv[i][0] <= end && end <= itv[i][1] {
				e = InsPos::Between(i,);
			} else if itv[i - 1][1] < end && end < itv[i][0] {
				e = InsPos::Left(i,);
			}
		}

		if start < itv[0][0] {
			s = InsPos::Left(0,);
		} else if itv[0][0] <= start && start <= itv[0][1] {
			s = InsPos::Between(0,);
		}
		if itv[0][0] <= end && end <= itv[0][1] {
			e = InsPos::Between(0,);
		} else if end < itv[0][0] {
			e = InsPos::Left(0,);
		}

		for i in 0..s.clone().unwrap(len,) {
			rslt.push(itv[i].clone(),);
		}

		let start = match s {
			InsPos::Between(i,) => itv[i][0],
			_ => start,
		};
		let end = match e {
			InsPos::Between(i,) => {
				e = InsPos::Between(i + 1,);
				itv[i][1]
			},
			_ => end,
		};

		rslt.push(vec![start, end],);
		for i in e.unwrap(len,)..len {
			rslt.push(itv[i].clone(),);
		}

		rslt
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 5], vec![6, 9]];
		let mut sol = Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5],);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
		let mut sol = Solution::insert(
			vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
			vec![4, 8],
		);
		assert_eq!(ans, sol);
	}
}

fn main() {}
