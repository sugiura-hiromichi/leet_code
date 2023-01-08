#![allow(unused)]

struct Solution;
impl Solution {
	pub fn is_match(s: String, p: String,) -> bool {
		let (plen, slen,) = (p.len(), s.len(),);
		// treat edge case
		if plen == 0 {
			return slen == 0;
		} else if slen == 0 {
			let mut i = 0;
			return !p.contains(|c| c != '*',);
		}
		// TODO: dp
		let mut dp = vec![vec![false; slen + 1]; plen + 1];
		dp[0][0] = true;

		for i in 1..=plen {
			for j in 1..=slen {
				let char_match = p[i - 1..i] == s[j - 1..j] || &p[i - 1..i] == "?";
				if &p[i - 1..i] == "*" {
					dp[i][j] = dp[i - 1][j - 1] || dp[i][j - 1] || dp[i - 1][j];
				} else if i > 1 && &p[i - 2..i - 1] == "*" {
					dp[i][j] = char_match && dp[i - 1][j] && (dp[i][j - 1] || dp[i - 1][j - 1]);
				} else {
					dp[i][j] = char_match && dp[i - 1][j - 1];
				}
			}
			if &p[i - 1..i] == "*" {
				dp[i][0] = dp[i - 1][0];
			}
		}

		dp[plen][slen]
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
