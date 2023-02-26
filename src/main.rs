// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn min_window(s: String, t: String,) -> String {
		let s_bytes = s.as_bytes();
		type ChGraph = std::collections::HashMap<u8, i32,>;
		let mut t_chars = t.bytes().fold(ChGraph::new(), |mut hm, b| {
			*hm.entry(b,).or_default() -= 1;
			hm
		},);

		let (mut rslt, mut l,) = ((0, 0,), 0,);
		for r in 1..=s.len() {
			match t_chars.get_mut(&s_bytes[r - 1],) {
				None => continue,
				Some(i,) => {
					*i += 1;
				},
			}

			while l < r {
				if let Some(i,) = t_chars.get_mut(&s_bytes[l],) {
					*i -= if *i > 0 { 1 } else { break };
				}
				l += 1;
			}

			if (0 == rslt.1 || r - l < rslt.1 - rslt.0) && t_chars.values().all(|&i| i >= 0,) {
				rslt = (l, r,);
			}
		}

		s[rslt.0..rslt.1].to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "BANC";
		let mut sol = Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "aa";
		let mut sol = Solution::min_window("aa".to_string(), "aa".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = "a";
		let mut sol = Solution::min_window("a".to_string(), "a".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = "";
		let mut sol = Solution::min_window("a".to_string(), "aa".to_string(),);
		assert_eq!(ans, sol);
	}

	// FAIL:
	#[test]
	fn test_4() {
		let mut ans = "babcbcacbbccbaccaaacacabbb";
		let mut sol = Solution::min_window(
			"acaabacababcbcacbbccbaccaaacacabbbcaccabccabbca".to_string(),
			"bbaacbacbbccbcaabbacbacac".to_string(),
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_slice() {
		let slice = "hehehe".to_string();
		assert_eq!(&slice[0..0], "");
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
