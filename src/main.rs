#![allow(unused)]

struct Solution;
impl Solution {
	pub fn is_match(s: String, p: String,) -> bool {
		let (mut si, mut pi, mut dp, mut asterisk, slen, plen,) = (0, 0, 0, None, s.len(), p.len(),);
		while si < slen {
			if pi < plen && (&p[pi..=pi] == "?" || p[pi..=pi] == s[si..=si]) {
				// advance both indexes
				si += 1;
				pi += 1;
			} else if pi < plen && &p[pi..=pi] == "*" {
				// `*` found. Only advance `p` index
				asterisk = Some(pi,);
				dp = si;
				pi += 1;
			} else if let Some(ast,) = asterisk {
				// last `p` index points '*'. advance `s` index
				pi = ast + 1;
				dp += 1;
				si = dp;
			} else {
				// current & last `p` index does not point '*'. Caracters do not match
				return false;
			}
		}

		while pi < plen && &p[pi..=pi] == "*" {
			pi += 1;
		}

		pi == plen
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = false;
		let mut sol = Solution::is_match("b".to_string(), "?*?".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = false;
		let mut sol = Solution::is_match("b".to_string(), "??".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_5() {
		let mut ans = true;
		let mut sol = Solution::is_match("abcabczzzde".to_string(), "*abc???de*".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_6() {
		let mut ans = true;
		let mut sol = Solution::is_match("ho".to_string(), "**ho".to_string(),);
		assert_eq!(ans, sol);
	}
}

fn main() { let mut sol = Solution::is_match("b".to_string(), "?*?".to_string(),); }
