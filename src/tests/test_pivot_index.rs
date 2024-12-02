#[test]
pub fn test_pivot_index() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let res = pivot_index(nums);
    println!("res: {}", res)
}

fn pivot_index(nums: Vec<i32>) -> i32 {
    let total = nums.iter().fold(0, |acc, x| acc + x);
    let mut left_sum = 0;
    for i in 0..nums.len() {
        let right_sum = total - left_sum - nums[i];
        if left_sum == right_sum {
            return i as i32;
        }
        left_sum += nums[i];
    }
    -1
}