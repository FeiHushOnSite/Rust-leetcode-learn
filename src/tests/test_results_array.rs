/// 找连续上升的段，
#[test]
pub fn test_results_array() {
    let nums = vec![1, 2, 3, 4, 3, 2, 5];
    let k = 3;

    let res = results_array(nums, k);
    println!("res: {:?}", res)
}

fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut cnt = 0;
    let mut ans = vec![-1; n - k as usize + 1];

    for i in 0..n {
        cnt = if i == 0 || nums[i] - nums[i - 1] != 1 { 1 } else { cnt + 1 };
        if cnt >= k {
            ans[i - k as usize + 1] = nums[i];
        }
    }
    ans
}