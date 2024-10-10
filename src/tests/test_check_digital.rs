#[test]
pub fn test_check_digital() {
    let s = String::from("cb34");
    let mut st = vec![];
    for x in s.bytes() {
        if x.is_ascii_digit() {
            st.pop();
        } else {
            st.push(x);
        }
    }
    unsafe {
        let res = String::from_utf8_unchecked(st);
        println!("res: {}", res);
    }
}
