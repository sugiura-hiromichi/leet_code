// •‿•
#![allow(unused)]
#![feature(test)]

struct Solution;
impl Solution {
	pub fn sort_colors(colors: &mut Vec<i32,>,) {
		let (mut cur0, mut cur1,) = (0, 0,);
		for i in 0..colors.len() {
			if colors[i] == 0 {
				if i != cur0 {
					if colors[cur0] == 1 {
						colors[i] = colors[cur1];
						colors[cur1] = 1;
					} else {
						colors[i] = colors[cur0];
					}
					colors[cur0] = 0;
				}
				cur0 += 1;
				cur1 += 1;
			} else if colors[i] == 1 {
				if i != cur1 {
					colors[i] = colors[cur1]; //`colors[cur1]` should be `2`
					colors[cur1] = 1;
				}
				cur1 += 1;
			}
			//println!("colors is {colors:?}");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![0, 0, 1, 1, 2, 2];
		let mut sol = vec![2, 0, 2, 1, 1, 0];
		Solution::sort_colors(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = vec![0, 1];
		let mut sol = vec![1, 0];
		Solution::sort_colors(&mut sol,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![0, 1, 2];
		let mut sol = vec![2, 0, 1];
		Solution::sort_colors(&mut sol,);
		assert_eq!(ans, sol);
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
