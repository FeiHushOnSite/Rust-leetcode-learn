#[test]
pub fn sum_digit_differences() {
    let mut nums = vec![13, 23, 12];
    let mut res = 0;
    let n = nums.len();
    while nums[0] > 0 {
        let mut cnt = vec![0; 10];
        for i in 0..n {
            cnt[nums[i] as usize % 10] += 1;
            nums[i] /= 10;
        }
        for j in 0..10 {
            res += (n - cnt[j]) as i64 * cnt[j] as i64;
        }
    }
    println!("{:?}", res / 2);
    let _ = res / 2;
}
