// •‿•
#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn combine(n: i32, k: i32,) -> Vec<Vec<i32,>,> {
		let mut rslt = vec![];
		if k == 1 {
			for i in 1..=n {
				rslt.push(vec![i],);
			}
			return rslt;
		}

		Self::combine(n, k - 1,).iter().for_each(|v| {
			for i in v[v.len() - 1] + 1..=n {
				let mut tmp = v.clone();
				tmp.push(i,);
				rslt.push(tmp,);
			}
		},);
		rslt
	}

	fn combination(n: i32, k: i32,) -> usize {
		let mut rslt = 1;
		for i in 0..k {
			rslt *= (n - i);
			rslt /= (i + 1);
		}
		rslt as usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]];
		let mut sol = Solution::combine(4, 2,);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1]];
		let mut sol = Solution::combine(1, 1,);
		ans.sort();
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn range_test() {
		for i in 5..=4 {
			assert!(true);
		}
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
			let mut h = std::collections::HashMap::new();
			for i in 0..100 {
				h.insert(i.to_string(), i,);
			}

			for i in 0..100 {
				*h.get_mut(&i.to_string(),).unwrap() += 1;
			}
		},)
	}

	#[bench]
	fn b2(b: &mut Bencher,) {
		b.iter(|| {
			let mut h = std::collections::HashMap::new();
			for i in 0..100 {
				h.insert(i.to_string(), i,);
			}

			for i in 0..100 {
				h.entry(i.to_string(),).and_modify(|i| *i += 1,);
			}
		},)
	}
}

fn main() {}
