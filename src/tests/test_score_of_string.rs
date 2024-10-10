#[test]
pub fn test_score_of_string() {
    let s = String::from("hello");
    let res = score_of_string(s);
    println!("res: {}", res);
    assert_eq!(res, 13)
}

fn score_of_string(s: String) -> i32 {
    s.as_bytes().windows(2).fold(0, |acc, window| {
        acc + (window[0] as i32 - window[1] as i32).abs()
    })
}
