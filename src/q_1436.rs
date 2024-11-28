use std::collections::HashMap;

pub fn solution() {
    let vec = s_1(vec![3, 2, 4], 6);
    println!("{:?}", vec);
    let vec = s_2(vec![3, 2, 4], 6);
    println!("{:?}", vec);
    let vec = s_3(vec![3, 2, 4], 6);
    println!("{:?}", vec);
    test_ownership();
}

fn s_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let enumerate1 = nums.iter().enumerate();
    let enumerate2 = nums.iter().enumerate();
    for (idx, num) in enumerate1 {
        println!("{:?}", enumerate2);
        for (idx_in, num_in) in enumerate2.clone() {
            if idx != idx_in && num + num_in == target {
                return vec![idx as i32, idx_in as i32];
            }
        }
    }
    Vec::new()
}

fn s_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    let enumerate = nums.iter().enumerate();
    for (i, num) in enumerate {
        let tar = target - num;
        if let Some(&idx) = num_map.get(&tar) {
            return vec![idx as i32, i as i32];
        }
        num_map.insert(*num, i);
    }
    Vec::new()
}

fn s_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(2);
    let len = nums.len();
    let mut idx_o = 0;
    while idx_o < len {
        let val_o = nums[idx_o];
        let mut idx_i = 0;
        while idx_i < len {
            let val_i = nums[idx_i];
            println!("{}, {}", val_i, val_o);
            if idx_o != idx_i && val_o + val_i == target {
                result.push(idx_o as i32);
                result.push(idx_i as i32);
                return result;
            }
            idx_i += 1;
        }
        idx_o += 1;
    }
    result
}
// -----------------------

fn test_ownership() {
    let t1 = T1 {};
    t1.test();
    // 下一行展示了self参数导致所有权进入新作用域后未返回后导致的结果，s1的enumerate2.into_iter正是如此
    // t1.test();
}

pub struct T1 {}

impl T1 {
    fn test(self) {
        print!("-----")
    }
}
