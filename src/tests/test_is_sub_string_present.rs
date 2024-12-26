#[test]
pub fn test_is_sub_sub_string_present() {
    let res = is_substring_present("leetcode".to_string());
    assert_eq!(true, res)
}

fn is_substring_present(s: String) -> bool {
    for i in 0..(s.len() - 1)  {
        let sub_str = &s[i..i + 2].chars().rev().collect::<String>();
        if s.contains(sub_str) {
            return true
        }
    }
    false
}
