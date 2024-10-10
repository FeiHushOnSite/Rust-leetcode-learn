#[test]
pub fn test_count_ways() {
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(1);
    // let mut nums = vec![1, 1];
    nums.sort();
    let n = nums.len();
    let mut res = 0;
    for i in 0..=n {
        if i > 0 && nums[i - 1] >= i as i32 {
            continue;
        }
        if i < n && nums[i] <= i as i32 {
            continue;
        }
        res += 1;
    }
    println!("res: {}", res);
}
