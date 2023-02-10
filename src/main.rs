// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn min_distance(w1: String, w2: String,) -> i32 {
		Self::rec(&w1, &w2, w1.len(), w2.len(),) as i32
	}

	fn rec(w1: &String, w2: &String, len1: usize, len2: usize,) -> usize {
		if len1 == 0 {
			len2
		} else if len2 == 0 {
			len1
		} else if w1[len1 - 1..len1] == w2[len2 - 1..len2] {
			Self::rec(w1, w2, len1 - 1, len2 - 1,)
		} else {
			let ins = Self::rec(w1, w2, len1, len2 - 1,);
			let del = Self::rec(w1, w2, len1 - 1, len2,);
			let rep = Self::rec(w1, w2, len1 - 1, len2 - 1,);
			ins.min(del.min(rep,),) + 1
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = 3;
		let mut sol = Solution::min_distance("horse".to_string(), "ros".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = 5;
		let mut sol = Solution::min_distance("intention".to_string(), "execution".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = 1;
		let mut sol = Solution::min_distance("a".to_string(), "".to_string(),);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

	//	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			// fastest vector init
			let mut v = vec![0; 1e5 as usize];
			for i in 0..1e5 as i32 {
				v[i as usize] = i;
			}
		},)
	}
}

fn main() {}
