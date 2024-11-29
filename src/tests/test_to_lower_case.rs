#[test]
pub fn test_to_lower_case() {
    let s = "HELLO".to_string();
    let res = to_lower_case(s);
    println!("res: {}", res)
}

fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}