use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    fn solve(&self) {
        
    }

}

fn s_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut l1 = l1;
    let mut l2 = l2;

    let mut carry = 0;
    while l1.is_some() || l2.is_some() {
        if let Some(node) = l1 {
            carry += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            carry += node.val;
            l2 = node.next;
        }
    }
    todo!()
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
        val,
        next: None,
    }
  }
}