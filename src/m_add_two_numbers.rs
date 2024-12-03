use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    fn solve(&self) {
        // [2,4,3]
        let mut node_1 = ListNode::new(2);
        let mut node_1_1 = Box::new(ListNode::new(4));
        let node_1_1_1 = Box::new(ListNode::new(3));
        node_1_1.next = Some(node_1_1_1);
        node_1.next = Some(node_1_1);
        // [5,6,4]
        let mut node_2 = ListNode::new(5);
        let mut node_2_1 = Box::new(ListNode::new(6));
        let node_2_1_1 = Box::new(ListNode::new(4));
        node_2_1.next = Some(node_2_1_1);
        node_2.next = Some(node_2_1);

        let option = s_1(Some(Box::new(node_1)), Some(Box::new(node_2)));
        println!("{:?}", option);
    }

    fn related_lang_concepts(&self) {
        println!("let some(y) = x {{}}，可以在x:Option<X>为Some时将Some中的值赋予y并作{{}}这个作用域中使用");
        println!("Option::as_mut()可以在Some时为其中的值提供可变引用的处理，如果直接&mut Option会要判断if Some");
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

    let mut result = ListNode::new(0);
    let mut cur_node = &mut result;
    let mut carry = 0;
    while l1.is_some() || l2.is_some() || carry != 0 {
        if let Some(node) = l1 {
            carry += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            carry += node.val;
            l2 = node.next;
        }
        cur_node.next = Some(Box::new(ListNode::new(carry % 10)));
        carry /= 10;
        cur_node = cur_node.next.as_mut()?;
    }
    result.next
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
