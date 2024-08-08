#![allow(unused)]
#![feature(test)]

/// doc
struct Solution;
impl Solution {
	pub fn add_binary(a: String, b: String,) -> String {
		let (al, bl,) = (a.len(), b.len(),);
		if al < bl {
			Self::add_binary(b, a,)
		} else {
			let mut a = a.to_owned();
			let mut increment = 0;
			for i in (0..al).rev() {
				if i < al - bl {
					// case of b is out of index
					match (&a[i..=i], increment,) {
						(_, 0,) => break,
						("0", 1,) => {
							a.replace_range(i..=i, "1",);
							increment = 0;
							break;
						},
						("1", 1,) => a.replace_range(i..=i, "0",),
						_ => unreachable!(),
					}
				} else {
					// case of b can be indexed
					assert!(i + bl >= al, "{}+{}>{}", i, bl, al);
					match (&a[i..=i], &b[i + bl - al..=i + bl - al],) {
						("0", "0",) => {
							if increment == 1 {
								a.replace_range(i..=i, "1",);
								increment = 0;
							}
						},
						("0", "1",) => {
							if increment == 0 {
								a.replace_range(i..=i, "1",);
							} else {
								increment = 1;
							}
						},
						("1", "0",) => {
							if increment == 1 {
								a.replace_range(i..=i, "0",);
							}
						},
						("1", "1",) => {
							if increment == 0 {
								a.replace_range(i..=i, "0",);
								increment = 1;
							} else {
								a.replace_range(i..=i, "1",);
							}
						},
						_ => unreachable!(),
					}
				}
			}
			if increment == 1 {
				a.insert(0, '1',);
			}
			a
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = "100";
		let mut sol = Solution::add_binary("11".to_string(), "1".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = "10101";
		let mut sol =
			Solution::add_binary("1010".to_string(), "1011".to_string(),);
		assert_eq!(ans, sol);
	}
}

mod benchs {
	extern crate test;
	use super::*;
	use test::black_box;
	use test::Bencher;

	#[bench]
	fn b1(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v.last(),);
			}
		},)
	}

	#[bench]
	fn b2(b: &mut Bencher,) {
		b.iter(|| {
			let v = vec![0; 100];
			for i in 0..1000 {
				black_box(v[v.len() - 1],);
			}
		},)
	}
}
