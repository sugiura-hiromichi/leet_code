// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn min_window(s: String, t: String,) -> String {
		let mut rslt = "".to_string();
		let mut l = match s.find(|c| t.contains(c,),) {
			Some(i,) if i < s.len() => i,
			_ => return rslt,
		};
		let mut r = if t.len() > 1 {
			match s[l + 1..].find(|c| t.contains(c,),) {
				Some(i,) => l + i + 1,
				None => return rslt,
			}
		} else {
			return t;
		};

		let mut t_chars = t.chars().fold(std::collections::HashMap::new(), |mut hm, c| {
			hm.entry(c.to_string(),).and_modify(|i| *i += 1,).or_insert(1,);
			hm
		},);
		for i in l..r {
			t_chars.entry(s[i..=i].to_string(),).and_modify(|i| *i -= 1,);
		}

		let s_len = s.len();
		let mut r_reached = false;
		// NOTE: `t_chars[&key]<0` means s[l..r] contains enough `key`
		// otherwise, `t_chars[&key]>0` means s[l..r] shortages `key`
		loop {
			if t_chars[&s[l..=l].to_string()] < 0 {
				t_chars.entry(s[l..=l].to_string(),).and_modify(|i| *i += 1,);
				l += 1 + s[l + 1..].find(|c| t.contains(c,),).unwrap();
			} else if r_reached {
				break;
			}

			if (r - l < rslt.len() || rslt.is_empty()) && t_chars.values().all(|&i| i <= 0,) {
				rslt = s[l..r].to_string();
				//if t_chars.values().all(|i| i == &0,) { break; }
			}

			if t_chars.values().any(|i| i > &0,) || t_chars[&s[l..=l].to_string()] >= 0 {
				match s[r..].find(|c| t.contains(c,),) {
					Some(i,) => {
						r += i + 1;
						t_chars.entry(s[r - 1..r].to_string(),).and_modify(|i| *i -= 1,);
					},
					None => {
						r_reached = true;
					},
				};
			}
			//println!( "{}{} {} {} {}", " ".repeat(l), s[l..r].to_string(),
			// t_chars[&"a".to_string()], t_chars[&"b".to_string()], t_chars[&"c".to_string()],);
		}

		//if t_chars.values().all(|i| i <= &0,) && r - l < rslt.len() { rslt = s[l..r].to_string();
		// }
		rslt
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
	fn test_3() {
		let mut ans = "";
		let mut sol = Solution::min_window("a".to_string(), "aa".to_string(),);
		assert_eq!(ans, sol);
	}

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
