#![allow(unused)]

struct Solution;
impl Solution {
	pub fn count_and_say(n: i32,) -> String {
		let mut ret = "1".to_string();
		for _ in 1..n {
			let mut tmp = "".to_string();
			loop {
				let mut i = 0;
				while i < ret.len() && ret[0..=0] == ret[i..=i] {
					i += 1;
				}

				tmp.push_str(&format!("{i}{}", &ret[0..=0]),);
				if i == ret.len() {
					ret = tmp;
					break;
				}
				ret = ret[i..].to_string();
			}
		}
		ret
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "1";
		let mut sol = Solution::count_and_say(1,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "1211";
		let mut sol = Solution::count_and_say(4,);
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
