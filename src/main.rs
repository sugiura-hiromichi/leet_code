#![allow(unused)]

struct Solution;
impl Solution {
	pub fn is_match(s: String, p: String,) -> bool {
		if p.len() == 0 || s.len() == 0 {
			return p.len() == s.len();
		}
		// TODO: dp
		let (plen, slen,) = (p.len(), s.len(),);
		let mut dp = vec![
			{
				let mut v = vec![false; slen];
				v.push(true,);
				v
			};
			plen
		];
		dp.push(vec![true; slen + 1],);

		for i in (0..plen).rev() {
			for j in (0..slen).rev() {
				let first_match = s[j..=j] == p[i..=i] || &p[i..=i] == "?" || &p[i..=i] == "*";

				if &p[i..=i] == "*" {
					// when `&p[i]=="*"`, s[j..]==p[i+1..]
					dp[i][j] = first_match && dp[i + 1][j];
				} else {
					dp[i][j] = first_match && dp[i + 1][j + 1];
				}
			}
		}

		dp[0][0]
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = false;
		let mut sol = Solution::is_match("aa".to_string(), "a".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = true;
		let mut sol = Solution::is_match("aa".to_string(), "*".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = false;
		let mut sol = Solution::is_match("cb".to_string(), "?a".to_string(),);
		assert_eq!(ans, sol);
	}
}

fn main() {}
