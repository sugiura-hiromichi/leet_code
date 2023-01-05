#![allow(unused)]

struct Solution;
impl Solution {
	pub fn multiply(num1: String, num2: String,) -> String {
		let (len1, len2,) = (num1.len(), num2.len(),);
		let mut digits = vec![0; len1 + len2];
		for (i1, c1,) in num1.chars().rev().enumerate() {
			for (i2, c2,) in num2.chars().rev().enumerate() {
				let n1 = c1 as u8 - '0' as u8;
				let n2 = c2 as u8 - '0' as u8;
				let rslt = n1 * n2 + digits[i1 + i2];
				digits[i1 + i2] = rslt % 10;
				digits[i1 + i2 + 1] += rslt / 10;
			}
		}

		while digits.len() > 1 && digits.last() == Some(&0,) {
			digits.pop();
		}

		digits.iter().rev().map(|d| d.to_string(),).collect()
	}
}

// t: This is Test Module
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "6";
		let mut sol = Solution::multiply("2".to_string(), "3".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "56088";
		let mut sol = Solution::multiply("123".to_string(), "456".to_string(),);
		assert_eq!(ans, sol);
	}
}

fn main() {}
