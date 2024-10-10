#[test]
pub fn test_count_special_numbers() {
    let s = 135;
    println!("{}", count_special_numbers(s));
}

pub fn count_special_numbers(n: i32) -> i32 {
    fn dfs(i: usize, mask: usize, is_limit: bool, is_num: bool, s:&[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == s.len() {
            return if is_num { 1 } else { 0 }; // is_num 为 true 表示得到了一个合法的数字
        }
        if !is_limit && is_num && memo[i][mask] != -1 {
            return memo[i][mask];
        }
        let mut res = 0;
        if !is_num { //可以跳过当前数位
            res = dfs(i + 1, mask, false, false, s, memo);
        }
        // 如果前面填的数字都和n的一样，那么这一位至多填数字 s[i] 否则就超过 n
        let up = if is_limit { s[i] - b'0' } else { 9 };
        // 枚举要填入的数字d
        // 如果前面没有填数字，则必须从 1 开始 因为不能有前导零
        let low = if is_num { 0 } else { 1 };
        for d in low..=up {
            if mask >> d & 1 == 0 { // d 不在mask中，说明之前没有填写过d
                res += dfs(i + 1, mask | (1 << d), is_limit && d == up, true, s, memo);
            }
        }
        if !is_limit && is_num {
            memo[i][mask] = res;
        }
        return res
    }
    let s = n.to_string();
    let s = s.as_bytes();
    let mut memo = vec![vec![-1; 1 << 10]; s.len()];
    return dfs(0, 0, true, false, &s, &mut memo)
}
