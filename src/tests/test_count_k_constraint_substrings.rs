/// 枚举所有子字符串的开始位置，从开始位置向右侧遍历，并且统计字符串内的o和1的个数
/// 如果符合k约束，就更新结果，如果不符合则跳出遍历，从下一个字符重新开始遍历
/// 最后可以遍历到所有符合条件的子字符串，返回它们的数量作为结果
#[test]
pub fn test_count_k_constraint_substrings() {
    let s = "10101".to_string();
    let res = count_k_constraint_substrings(s, 1);
    println!("res: {}", res)
}

fn count_k_constraint_substrings(s: String, k:i32) -> i32 {
    let n = s.len();
    let s: Vec<u8> = s.bytes().collect();
    let mut res = 0;
    for i in 0..n {
        let mut count = [0, 0];
        for j in i..n {
            count[s[j] as usize - b'0' as usize] += 1;
            if count[0] > k && count[1] > k {
                break;
            }
            res += 1;
        }
    }
    res
}