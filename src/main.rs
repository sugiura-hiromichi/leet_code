// •‿•
#![allow(unused)]
#![feature(test)]

extern crate test;
use test::black_box;
use test::Bencher;

struct Solution;
impl Solution {
	pub fn full_justify(words: Vec<String,>, max_width: i32,) -> Vec<String,> {
		let mut rslt = vec!["".to_string()];

		let mut len = 0;
		for s in words {
			len += s.len();
			if len > max_width as usize {
				len = s.len();
				if let Some(s,) = rslt.last_mut() {
					if &s[s.len() - 1..s.len()] == " " {
						s.pop();
					}
				}
				rslt.push(s,);
			} else {
				rslt.last_mut().unwrap().push_str(&s,);
			}

			if len < max_width as usize {
				rslt.last_mut().unwrap().push(' ',);
				len += 1;
			}
		}

		// modify last element
		let mut last = rslt.pop().unwrap();
		let llen = last.len();
		last.push_str(&" ".repeat(max_width as usize - llen,),);

		// modify other elements
		rslt.iter_mut().for_each(|s| {
			let spaces = max_width as usize - s.len();
			let gaps = s.split_whitespace().count() - 1;
			if gaps != 0 {
				let mut ext = spaces % gaps;
				let pad = spaces / gaps;
				let mut i = 0;
				while i < max_width as usize {
					if &s[i..=i] == " " {
						if ext != 0 {
							ext -= 1;
							s.insert_str(i, &" ".repeat(pad + 1,),);
						} else {
							s.insert_str(i, &" ".repeat(pad,),);
						}
						i += pad;
						while &s[i..=i] == " " {
							i += 1;
						}
					} else {
						i += 1;
					}
				}
			} else {
				s.push_str(&" ".repeat(spaces,),);
			}
		},);

		rslt.push(last,);
		rslt
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec!["This    is    an", "example  of text", "justification.  "];
		let mut sol = Solution::full_justify(
			vec![
				"This".to_string(),
				"is".to_string(),
				"an".to_string(),
				"example".to_string(),
				"of".to_string(),
				"text".to_string(),
				"justification.".to_string(),
			],
			16,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec!["What   must   be", "acknowledgment  ", "shall be        "];
		let mut sol = Solution::full_justify(
			vec![
				"What".to_string(),
				"must".to_string(),
				"be".to_string(),
				"acknowledgment".to_string(),
				"shall".to_string(),
				"be".to_string(),
			],
			16,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![
			"Science  is  what we",
			"understand      well",
			"enough to explain to",
			"a  computer.  Art is",
			"everything  else  we",
			"do                  ",
		];
		let mut sol = Solution::full_justify(
			vec![
				"Science".to_string(),
				"is".to_string(),
				"what".to_string(),
				"we".to_string(),
				"understand".to_string(),
				"well".to_string(),
				"enough".to_string(),
				"to".to_string(),
				"explain".to_string(),
				"to".to_string(),
				"a".to_string(),
				"computer.".to_string(),
				"Art".to_string(),
				"is".to_string(),
				"everything".to_string(),
				"else".to_string(),
				"we".to_string(),
				"do".to_string(),
			],
			20,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = vec![
			"ours  to see Que sera sera When I was just a child in school",
			"I                                                           ",
		];
		let mut sol = Solution::full_justify(
			vec![
				"ours".to_string(),
				"to".to_string(),
				"see".to_string(),
				"Que".to_string(),
				"sera".to_string(),
				"sera".to_string(),
				"When".to_string(),
				"I".to_string(),
				"was".to_string(),
				"just".to_string(),
				"a".to_string(),
				"child".to_string(),
				"in".to_string(),
				"school".to_string(),
				"I".to_string(),
			],
			60,
		);
		assert_eq!(ans, sol);
	}

	#[test]
	fn usize_from_bool() {
		assert_eq!(0, usize::from(false));
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

fn main() { assert_eq!(21 - 15 % 6, 18) }
