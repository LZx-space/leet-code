use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    fn solve(&self) {
        let i = s_1("abcabcbb".to_string());
        println!("longest substring len {}", i);
    }
}

fn s_1(s: String) -> i32 {
    s.len() as i32
}