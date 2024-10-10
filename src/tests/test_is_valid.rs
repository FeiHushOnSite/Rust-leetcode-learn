use std::collections::HashMap;
#[test]
pub fn test_is_valid() {
    let str = "(([[]]}}".to_string();
    let res = is_valid(str);
    println!("string is valid: {}", res);
    let is_valid = is_valid_sec("()".to_string());
    println!("string is valid: {}", is_valid);
}

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 { // s 长度必须是偶数
        return false;
    }
    let mp = [(b')', b'('), (b']', b'['), (b'}', b'{')].iter().cloned().collect::<HashMap<_, _>>();
    let mut st = vec![]; //为了方便 Rust 提供了宏，这个宏会根据我们提供的值来创建新的vector
    for c in s.bytes() {
        if !mp.contains_key(&c) { //c 是左括号
            st.push(c)
        } else if st.is_empty() || st.pop().unwrap() != *mp.get(&c).unwrap() {
            return false; //没有左括号，或者左括号类型不对
        }
    }
    st.is_empty()
}

pub fn is_valid_sec(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let mut st = vec![];
    for c in s.bytes() {
        match c {
            b'(' => st.push(b')'), // 入栈对应右括号
            b'{' => st.push(b'}'),
            b'[' => st.push(b']'),
            _ => { // c是右括号
                if st.pop() != Some(c) {
                    return false; //没有左括号，或者左括号类型不对
                }
            }
        }
    }
    st.is_empty() // 所有左括号必须匹配完毕
}
