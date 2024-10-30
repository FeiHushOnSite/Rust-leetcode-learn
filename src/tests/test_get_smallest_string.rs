#[test]
pub fn test_get_smallest_string() {
    let s = "45320".to_string();
    let res = get_smallest_string(s);
    println!("res:{:?}", res)
}

fn get_smallest_string(s: String) -> String {
    let mut arr: Vec<char> = s.chars().collect();
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] && (arr[i] as i32 % 2) == (arr[i + 1] as i32) % 2 {
            arr.swap(i, i + 1);
            break;
        }
    }
    arr.iter().collect()
}