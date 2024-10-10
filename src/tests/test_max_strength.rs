#[test]
pub fn max_strength() {
    let nums = vec![3, -1, -5, 2, 5, -9];
    let mut mn: i64 = nums[0];
    let mut mx: i64 = mn;
    for &x in &nums[1..] {
        let x = x;
        let tmp = mn;
        mn = mn.min(x).min(mn * x).min(mx * x);
        mx = mx.max(x).max(tmp * x).max(mx * x);
    }
    println!("max is {}", mx)
}
