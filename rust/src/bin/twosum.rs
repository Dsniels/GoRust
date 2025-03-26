use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, value) in nums.iter().enumerate() {
        let complement = target - value;
        map.insert(complement, i as i32);
    }

    for (i, num) in nums.iter().enumerate() {
        if let Some(value) = map.get(&num) {
            if *value != i as i32 {
                return vec![i as i32, *value];
            }
        }
    }
    return vec![];
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;



    let ans = two_sum(nums, target);
    print!("{:?}", ans);
}
