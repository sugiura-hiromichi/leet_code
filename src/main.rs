// •‿•
#![allow(unused)]
struct Solution;
use std::cmp::Ordering;
impl Solution {
	pub fn insert(itv: Vec<Vec<i32,>,>, new_itv: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let len = itv.len();
		let (mut s, mut e,) = (new_itv[0], new_itv[1],);

		let li = itv.binary_search_by(Self::pred(s,),).unwrap_or_else(|x| x,);
		let ri = itv.binary_search_by(Self::pred(e,),).unwrap_or_else(|x| x,);
		let l = &itv[..(li + usize::from(li < len && itv[li][1] < s,))];
		let r = &itv[(ri + usize::from(ri >= len || itv[ri][0] <= e,)).min(len,)..];

		// q: what case is cared about in this `if`?
		// -> this is required to determine range of `vec![s, e]`
		if l.len() + r.len() != len {
			s = s.min(itv[l.len()][0],);
			e = e.max(itv[len - r.len() - 1][1],);
		}

		vec![l, &vec![vec![s, e]], r].concat()
	}

	fn pred(x: i32,) -> impl Fn(&Vec<i32,>,) -> Ordering {
		move |i: &Vec<i32,>| {
			if i[0] > x {
				Ordering::Greater
			} else if i[1] < x {
				Ordering::Less
			} else {
				Ordering::Equal
			}
		}
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

	#[test]
	fn usize_from() {
		assert_eq!(usize::from(true), 1);
		assert_eq!(usize::from(false), 0);
	}
}

fn main() {}

