#![allow(unused)]

struct Solution;
impl Solution {
	pub fn combination_sum(candidates: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
		let mut ret = std::collections::HashSet::new();
		let mut n = 1;
		while n <= target / 2 {
			let n_sum = Self::combination_sum(candidates.clone(), n,);
			let t_sub_n = Self::combination_sum(candidates.clone(), target - n,);

			if !n_sum.is_empty() && !t_sub_n.is_empty() {
				for n in n_sum {
					for t_n in t_sub_n.iter() {
						let mut v = [n.clone(), t_n.clone(),].concat();
						v.sort();
						ret.insert(v,);
					}
				}
			}
			n += 1;
		}

		for i in 0..candidates.len() {
			if candidates[i] == target {
				ret.insert(vec![target],);
				break;
			}
		}
		ret.iter().map(|v| v.clone(),).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![2, 2, 3], vec![7]];
		let mut sol = Solution::combination_sum(vec![2, 3, 6, 7], 7,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
		let mut sol = Solution::combination_sum(vec![2, 3, 5], 8,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans: Vec<Vec<i32,>,> = vec![];
		let mut sol = Solution::combination_sum(vec![2], 1,);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans: Vec<Vec<i32,>,> = vec![];
		let mut sol = Solution::combination_sum(vec![5, 10, 8, 4, 3, 12, 9], 27,);
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
