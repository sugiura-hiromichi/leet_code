// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn simplify_path(mut p: String,) -> String {
		p.push('/',);
		// treat duplicated slash
		Self::remove_dup_slash(&mut p,);
		// remove `./`
		Self::remove_explicit_curdir(&mut p,);
		// remove `../`
		Self::remove_explicit_parent_dir(&mut p,);

		if p.len() == 1 {
			p
		} else {
			p.pop();
			p
		}
	}

	fn remove_dup_slash(p: &mut String,) {
		let mut i = 0;
		while i < p.len() - 1 {
			if &p[i..=i] == "/" {
				let mut j = i + 1;
				while j < p.len() && &p[j..=j] == "/" {
					j += 1;
				}
				p.replace_range(i + 1..j, "",);
			}
			i += 1;
		}
	}

	fn remove_explicit_curdir(p: &mut String,) {
		let mut i = 0;
		while p.len() > 2 && i < p.len() - 2 {
			if &p[i..i + 3] == "/./" {
				p.replace_range(i..i + 2, "",);
			} else {
				i += 1;
			}
		}
	}

	fn remove_explicit_parent_dir(p: &mut String,) {
		let mut i = 0;
		while p.len() > 3 && i < p.len() - 3 {
			if &p[i..i + 4] == "/../" {
				if i == 0 {
					p.replace_range(i..i + 3, "",);
				} else {
					let start = p[..i].rfind('/',).unwrap();
					p.replace_range(start..i + 3, "",);
					i = start;
				}
			} else {
				i += 1;
			}
		}
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

fn main() {
	let root = Solution::simplify_path("/.././.../././".to_string(),);
	assert_eq!("/...", root);
}
