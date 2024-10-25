#[test]
pub fn test_min_operations() {
    let nums = vec![0, 1, 1, 1, 0 ,0];

    let res = min_operations(nums);

    println!("res: {}", res);
}


fn min_operations(mut nums: Vec<i32>) -> i32 {
    let n  = nums.len();
    let mut ans = 0;
    let mut i = 0;
    while i + 2 < n {
       if nums[i] == 0 {
           ans += 1;
           nums[i + 1] ^= 1;
           nums[i + 2] ^= 1;
       }
        i += 1;
    }
    if nums[n - 1] == 0 || nums[n - 2] == 0 {
        return -1;
    }
    ans
}


// fn min_operations(mut nums: Vec<i32>) -> i32 {
//     let mut q = std::collections::VecDeque::<usize>::new();
//
//     let mut res = 0;
//     let len = nums.len();
//
//     for(i, n) in nums.into_iter().enumerate() {
//         while let Some(j) = q.front() {
//             if j + 2 < i {
//                 q.pop_front();
//             } else {
//                 break;
//             }
//         }
//         if n % 2 == q.len() as i32 % 2 {
//             if i + 2 >= len {
//                 return -1;
//             }
//             q.push_back(i);
//             res += 1;
//         }
//     }
//     res
// }

// fn min_operations(mut nums: Vec<i32>) -> i32 {
//     let mut index = 0;
//     for i in 0..nums.len() {
//         if nums[i] == 0 {
//             nums[i] ^= 1;
//             nums[i + 1] ^= 1;
//             nums[i + 2] ^= 1;
//             index += 1;
//         }
//     }
//     println!("nums: {:?}", nums);
//     index as i32
// }