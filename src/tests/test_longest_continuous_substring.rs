#[test]
fn test_longest_continuous_substring() {
    let res = longest_continuous_substring(String::from("abacaba"));
    println!("res: {}", res)
}

fn longest_continuous_substring(s: String) -> i32 {
    let mut count = 1;
    let mut res = 1;
    let s = s.as_bytes();
    for i in 0.. s.len() - 1 {
        if s[i + 1] == s[i] + 1 {
            count += 1;
            res = res.max(count)
        } else {
            count = 1;
        }
    }
    res
}
