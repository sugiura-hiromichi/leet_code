#![allow(unused)]

struct Solution;
impl Solution {
	pub fn combination_sum2(candidates: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
		let mut can = candidates;
		can.sort();
		core_sum(can, target,)
	}
}

fn core_sum(can: Vec<i32,>, target: i32,) -> Vec<Vec<i32,>,> {
	let (mut ret, len,) = (vec![], can.len(),);
	if can.is_empty() || target < can[0] {
		return ret;
	}

	let mut same = 1;
	while same < len && can[0] == can[same] {
		same += 1;
	}

	if target % can[0] == 0 && target / can[0] <= same as i32 {
		ret.push(vec![can[0]; (target / can[0]) as usize],);
	}

	for n in 0..=same {
		for mut v in core_sum(can[same..].to_vec(), target - n as i32 * can[0],) {
			for _ in 0..n {
				v.push(can[0],);
			}
			ret.push(v,);
		}
	}
	ret
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
		let mut sol = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = vec![vec![1, 2, 2], vec![5]];
		let mut sol = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans: Vec<Vec<i32,>,> = vec![];
		let mut sol = Solution::combination_sum2(vec![1], 2,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans =
			vec![vec![1, 1, 2, 2], vec![1, 1, 4], vec![1, 2, 3], vec![2, 2, 2], vec![2, 4]];
		let mut sol = Solution::combination_sum2(vec![4, 4, 2, 1, 4, 2, 2, 1, 3], 6,);
		sol.iter_mut().for_each(|v| v.sort(),);
		sol.sort();
		assert_eq!(ans, sol);
	}
}

fn main() {
	println!("168 is {:b}", 168);
}
