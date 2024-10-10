#[test]
pub fn test_stable_mountains() {
    let height = vec![10,1,10,1,10];
    let threshold = 10;
    let res = stable_mountains(height, threshold);
    println!("res is {:?}", res)
}

fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let n = height.len();
    let mut res:Vec<i32> = Vec::new();
    for i in 1..n {
        if height[i - 1] > threshold {
            res.push(i as i32);
        }
    }
    res
}
