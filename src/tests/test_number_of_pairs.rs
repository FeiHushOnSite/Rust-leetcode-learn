use std::collections::HashMap;

#[test]
pub fn test_number_of_pairs() {
    let nums1 = vec![1, 3, 4];
    let nums2 = vec![1, 3, 4];
    let res = number_of_pairs(nums1, nums2, 1);
    assert_eq!(5, res);
    let res = number_of_pairs_enum(vec![1, 2, 4, 12], vec![2, 4], 3);
    assert_eq!(2, res);
}

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    for num1 in &nums1 {
        for num2 in &nums2 {
            if num1 % (num2 * k) == 0 {
                count += 1;
            }
        }
    }
    count
}

/**
1. 统计 a[i] / k 的 出现次数，保存到哈希表cnt1中
2. 统计 b[j] 的出现次数(相同b[j]无需重复计算)，保存到哈希表cnt2中
3. 设 cnt1 中最大key为u
4. 枚举cnt2中元素x，然后枚举x 的倍数 y = x, 2x, 3x, ...不超过u，累加cnt1[y]，再乘以cnt2[x]
**/
fn number_of_pairs_enum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut cnt1 = HashMap::new();
    for x in nums1 {
        if x % k == 0 {
            *cnt1.entry(x / k).or_insert(0) += 1;
        }
    }

    if cnt1.is_empty() {
        return 0;
    }
    let mut cnt2 = HashMap::new();
    for x in nums2 {
        *cnt2.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    let mx = *cnt1.keys().max().unwrap();
    for(x, cnt) in cnt2 {
        let mut s = 0;
        // step_by()：按步长跳跃访问迭代器元素。第一个元素总会被访问。若步长为1，则访问每一个元素；步长为2，则每间隔一个元素访问。以此类推。
        for y in (0..=mx).step_by(x as usize) {
            if let Some(&c) = cnt1.get(&y) {
                s += c;
            }
        }
        ans += s as i64 * cnt as i64
    }
    ans
}
