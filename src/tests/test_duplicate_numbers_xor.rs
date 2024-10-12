use std::collections::HashMap;

#[test]
pub fn test_duplicate_numbers_xor() {
    let target = vec![1, 2, 2, 1];
    let res = duplicate_numbers_xor(target);
    println!("res: {}", res);
}

fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut cnt = HashMap::new();
    for num in nums {
        *cnt.entry(num).or_insert(0) += 1;
    }
    // println!("map: {:?}", cnt);
    for (k , v) in cnt {
        if v > 1 {
            res ^= k;
        }
    }
    res
}
