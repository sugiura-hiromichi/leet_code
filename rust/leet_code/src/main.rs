#![allow(unused, dead_code)]
#[derive(PartialEq, Eq, Clone, Debug,)]
pub struct ListNode {
	pub val:  i32,
	pub next: Option<Box<ListNode,>,>,
}

impl ListNode {
	#[inline]
	fn new(val: i32,) -> Self { ListNode { next: None, val, } }
}

struct Solution;
impl Solution {
	pub fn merge_k_lists(lists: Vec<Option<Box<ListNode,>,>,>,) -> Option<Box<ListNode,>,> {
		let mut v = lists.iter().flat_map(|opt| list_to_ary(opt.clone(),),).collect::<Vec<i32,>>();
		v.sort();
		ary_to_list(v,)
	}
}

fn ary_to_list(ary: Vec<i32,>,) -> Option<Box<ListNode,>,> {
	if ary.is_empty() {
		None
	} else {
		Some(Box::new(ListNode { val: ary[0], next: ary_to_list(ary[1..].to_vec(),), },),)
	}
}

fn list_to_ary(mut list: Option<Box<ListNode,>,>,) -> Vec<i32,> {
	let mut ret = vec![];
	while let Some(ref nod,) = list {
		ret.push(nod.clone().val,);
		list = list.unwrap().next;
	}
	ret
}

fn merge_two_lists(
	l1: Option<Box<ListNode,>,>,
	l2: Option<Box<ListNode,>,>,
) -> Option<Box<ListNode,>,> {
	match (l1.clone(), l2.clone(),) {
		(Some(mut one,), Some(mut two,),) => {
			if one.val < two.val {
				Some(Box::new(ListNode { val: one.val, next: merge_two_lists(one.next, l2,), },),)
			} else {
				Some(Box::new(ListNode { val: two.val, next: merge_two_lists(two.next, l1,), },),)
			}
		},
		(nxt, None,) | (None, nxt,) => nxt,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let mut ans = ary_to_list(&[1, 1, 2, 3, 4, 4, 5, 6,],);
		let mut sol =
			Solution::merge_k_lists(arys_to_lists(vec![&[1, 4, 5,], &[1, 3, 4,], &[2, 6,]],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let mut ans = ary_to_list(&[],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![&[]],),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_4() {
		let mut ans = ary_to_list(&[1, 2, 3,],);
		let mut sol = Solution::merge_k_lists(arys_to_lists(vec![&[1,], &[2, 3,]],),);
		assert_eq!(ans, sol);
	}

	fn ary_to_list(ary: &[i32],) -> Option<Box<ListNode,>,> {
		if ary.is_empty() {
			None
		} else {
			Some(Box::new(ListNode { val: ary[0], next: ary_to_list(&ary[1..],), },),)
		}
	}

	fn arys_to_lists(arys: Vec<&[i32],>,) -> Vec<Option<Box<ListNode,>,>,> {
		arys.iter().map(|&a| ary_to_list(a,),).collect()
	}
}

fn main() {}
