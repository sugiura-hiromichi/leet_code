// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn simplify_path(path: String,) -> String {
		let mut p = path.split('/',);
		let mut valid_path = vec![];
		let mut rslt = String::new();

		p.for_each(|s| {
			if !valid_path.is_empty() && s == ".." {
				valid_path.pop();
			} else if s != "" && s != "." && s != ".." {
				valid_path.push(s,);
			}
		},);

		if valid_path.is_empty() {
			rslt.push('/',);
		} else {
			valid_path.iter().for_each(|s| {
				rslt.push('/',);
				rslt.push_str(s,);
			},);
		}
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "/home";
		let mut sol = Solution::simplify_path("/home/".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "/";
		let mut sol = Solution::simplify_path("/.".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = "/home/foo";
		let mut sol = Solution::simplify_path("/home//foo/".to_string(),);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	use super::*;

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
