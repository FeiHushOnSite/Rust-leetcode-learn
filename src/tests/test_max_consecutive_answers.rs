use std::cmp::max;

#[test]
pub fn max_consecutive_answers() {
    let answer_key: String = String::from("TTFTTFTT");
    let answer1 = answer_key.clone();
    let answer2 = answer_key.clone();
    // let s = answer_key.chars().collect::<Vec<char>>();
    let k = 2;
    let res = max(max_consecutive_char(answer1, k, 'T'), max_consecutive_char(answer2, k, 'F'));
    println!("max consecutive answer is : {}", res)
}

fn max_consecutive_char(answer_key: String, k: i32, ch: char) -> i32 {
    let n = answer_key.len();
    let (mut res, mut left, right, mut sum) = (0, 0, 0, 0);
    for right in right..n {
        sum += if answer_key.chars().nth(right) != Some(ch) { 1 } else { 0 };
        while sum > k {
            sum -= if answer_key.chars().nth(left) != Some(ch) { 1 } else { 0 };
            left += 1;
        }
        res = max(res, (right - left + 1) as i32);
    }
    return res;
}
