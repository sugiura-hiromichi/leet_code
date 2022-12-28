#![allow(unused)]

struct Solution;
impl Solution {
	pub fn combination_sum(candidates: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
		let mut can = candidates;
		can.sort();
		core_sum(can, target,)
	}
}

fn core_sum(can: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
	let mut ret = vec![];
	let mut i = 0;
	while i + 1 < can.len() && can[i] < target {
		for n in 1..=target / can[i] {
			if can[i] * n == target {
				ret.push(vec![can[i]; n as usize],);
			} else {
				core_sum(can[i + 1..].to_vec(), target - n * can[i],).iter_mut().for_each(|v| {
					for _ in 0..n {
						v.push(can[i],);
					}
					ret.push(v.clone(),);
				},);
			}
		}

		i += 1;
	}

	if 0 == target % can[i] {
		ret.push(vec![can[i]; (target / can[i]) as usize],);
	}

	ret
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![2, 2, 3], vec![7]];
		let mut sol = Solution::combination_sum(vec![2, 3, 6, 7], 7,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	// FAIL: `sol==[3,5]`
	#[test]
	fn test_2() {
		let mut ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
		let mut sol = Solution::combination_sum(vec![2, 3, 5], 8,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans: Vec<Vec<i32,>,> = vec![];
		let mut sol = Solution::combination_sum(vec![2], 1,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans: Vec<Vec<i32,>,> = vec![vec![2, 2, 2], vec![3, 3]];
		let mut sol = Solution::combination_sum(vec![2, 3], 6,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
