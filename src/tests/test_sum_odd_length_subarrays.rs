#[test]
pub fn test_sum_odd_length_sub_arrays() {
    let arr = vec![1, 4, 2, 5, 3];
    let res = sum_odd_length_sub_arrays(arr);
    assert_eq!(58, res);
}
fn sum_odd_length_sub_arrays(arr: Vec<i32>) -> i32 {
    let mut sum = 0;
    let n = arr.len();
    for start in 0..n {
        let mut length = 1;
        while start + length <= n {
            for v in &arr[start..start + length] {
                sum += v
            }
            length += 2
        }
    }
    sum
}

// fn sum_odd_length_sub_arrays(arr: Vec<i32>) -> i32 {
//     let mut sum = 0;
//     let n = arr.len();
//     for i in 0..n {
//         let left_count = i;
//         let right_count = n - i - 1;
//         let left_odd = (left_count + 1) / 2;
//         let right_odd = (right_count + 1) / 2;
//         let left_even = left_count / 2 + 1;
//         let right_even = right_count / 2 + 1;
//         sum += arr[i] * (left_odd * right_odd + left_even * right_even) as i32;
//     }
//     sum
// }