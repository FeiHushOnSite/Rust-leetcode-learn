#[test]
pub fn test_title_to_number() {

    let s = "AB".to_string();
    let mut res = 0u8;
    for c in s.chars() {
        let n = c.to_ascii_uppercase() as u8 - b'A' + 1;
        res = res * 26 + n;
    }
    println!("res: {}", res);

    let v = vec![1, 2, 3];
    let v_even = v.iter().fold(0, |acc, x| acc + x);
    println!("v: {}", v_even);

    let res1 = title_to_number(s);
    println!("res1: {}", res1);
}

fn title_to_number_extra(column_title: String) -> i32 {
    column_title.as_bytes().iter().fold(0, |ans, c| ans * 26 + (c - (b'A' - 1)) as i32)
}

fn title_to_number(column_title: String) -> i32 {
    let mut res = 0;
    for &i in column_title.as_bytes(){
        res = res * 26 + (i.to_ascii_uppercase() - (b'A' - 1)) as i32;
    }
    res
}