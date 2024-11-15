use std::collections::HashMap;

#[test]
pub fn test_num_jewels_in_stones() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();

    let res = num_jewels_in_stones(jewels, stones);
    println!("res: {}", res)
}


fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count = 0;
    jewels.chars().for_each(|e| count = *count_chars(stones.clone()).get(&e).unwrap_or(&0));
    count as i32
}

fn count_chars(s: String) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    s.chars().for_each(|e| *counts.entry(e).or_insert(1) += 1);
    counts
}
// fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
//     let mut res = 0;
//
//     for i in jewels.chars() {
//         for j in stones.chars() {
//             if i == j {
//                 res = res + 1;
//             }
//         }
//     }
//     res
// }