use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    fn solve(&self) {
        let i = s_1(&mut vec![1, 1, 1, 1, 2, 4]);
        println!("--{}", i);
    }

}

fn s_1(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 2;
    while idx < nums.len() {
          if nums[idx] == nums[idx - 2] {
              nums.remove(idx);
          } else { 
              idx += 1;
          }
    }
    nums.len() as i32
}