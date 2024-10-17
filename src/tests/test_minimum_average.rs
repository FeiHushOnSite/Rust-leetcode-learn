#[test]
pub fn test_minimum_average() {
    let nums = vec![7,8,3,4,15,13,4,1];
    let res = minimum_average(nums);
    println!("res: {}", res)
}

fn minimum_average(nums: Vec<i32>) -> f64 {
    let mut avgs:Vec<f64> = Vec::new();
    let mut min_element = 0.0;
    let mut max_element = 0.0;

    let len = nums.len();

    let mut nums = nums.clone();
    nums.sort();
    for i in 0..len {
        min_element = nums[i] as f64;
        max_element = nums[len - i - 1usize] as f64;
        avgs.push(((max_element + min_element) / 2.0));
    }
    avgs.into_iter()
        .reduce(f64::min)
        .unwrap()
}
