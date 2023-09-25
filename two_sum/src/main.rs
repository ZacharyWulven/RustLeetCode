use std::collections::HashMap;

fn main() {

    let list = two_sum(vec![11, 2, 7, 15], 9);
    println!("{:?}", list);
}

  
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut the_map:HashMap<i32, i32> = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(idx) = the_map.get(&diff) {
            return vec![*idx, index as i32];
        }
        the_map.insert(num, index as i32);
    }
    vec![]
}


