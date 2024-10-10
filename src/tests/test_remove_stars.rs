#[test]
pub fn test_remove_stars() {
    let s = String::from("leet**cod*e");
    let mut res = vec![];
    for c in s.bytes() {
        if c == b'*' {
            res.pop();
        } else {
            res.push(c);
        }
    }
    let result = unsafe { String::from_utf8_unchecked(res) };
    println!("{:?}", result)
}
