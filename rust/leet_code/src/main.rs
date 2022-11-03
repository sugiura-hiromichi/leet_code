struct Solution;
impl Solution {
	pub fn three_sum(nums: Vec<i32,>,) -> Vec<Vec<i32,>,> {
		let mut nlist = std::collections::HashMap::new();
		let mut nums: Vec<i32,> = nums
			.iter()
			.filter_map(|&k| {
				if let Some(v,) = nlist.insert(k, 1,) {
					if v < 3 {
						nlist.insert(k, v + 1,);
						Some(k,)
					} else {
						None
					}
				} else {
					None
				}
			},)
			.collect();
		nums.sort();

		// FIXME:
		let mut veclist = std::collections::HashSet::new();
		for i in 0..nums.len() - 2 {
			for j in i + 1..nums.len() - 1 {
				let k = -nums[i] - nums[j];
				if k < 0 || nums[i] + nums[j] * 2 > 0 {
					break;
				}
				if let Some(v,) = nlist.get(&k,) {
					if k == nums[i] {
						if k == nums[j] {
							if *v > 2 {
								veclist.insert(vec![nums[i], nums[j], k],);
							}
						} else if *v > 1 {
							veclist.insert(vec![nums[i], nums[j], k],);
						}
					} else {
						veclist.insert(vec![nums[i], nums[j], k],);
					}
				}
			}
		}

		let mut vec = vec![];
		for v in veclist {
			vec.push(v,);
		}

		vec
	}
}

fn main() {
	assert_eq!(
		Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
		vec![vec![-1, -1, 2], vec![-1, 0, 1],]
	);
	println!("1----");
	assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}
