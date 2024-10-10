#[test]
pub fn test_difference_of_sum() {
    let target = vec![1, 2, 3, 4, 5];
    let res = difference_of_sum(target);
    println!("res: {}", res)
}

fn difference_of_sum(nums: Vec<i32>) -> i32 {
    nums.iter().sum::<i32>() - nums.into_iter().map(|e| sum_digital(e)).collect::<Vec<i32>>().iter().sum::<i32>()
}

fn sum_digital(num: i32) -> i32 {
    if num == 0 {
        return num
    }
    num % 10 + sum_digital(num / 10)
}
