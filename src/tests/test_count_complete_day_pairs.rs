#[test]
pub fn test_count_complete_day_pairs() {
    let hours = vec![12, 12, 30, 24, 24];
    let res = count_complete_day_pairs(hours);
    println!(
        "res: {}", res
    );

    println!("lll: {}", 24 % 24)
}
// fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
//     let mut map = [0; 24];
//
//     let mut res = 0;
//     hours.iter().for_each(|&h| {
//         let t = (h % 24) as usize;
//         res += map[(24 - t) % 24];
//         map[t] += 1;
//     });
//
//     res
// }
fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    // 用于存储每个余数出现的次数
    let mut count = vec![0; 24];
    let mut res = 0;

    //统计每个余数的出现次数
    for &h in hours.iter() {
        let remainder = (h % 24) as usize;
        count[remainder] += 1;
    }

    println!("count: {:?}", count);

    // 处理余数为 0 的情况，C (count[0], 2) = count[0] * count[0] - 1 / 2
    res += count[0] * (count[0] - 1) / 2;

    // 处理余数为 12 的情况， 因为12 + 12 = 24， C(count[12], 2)
    res += count[12] * (count[12] - 1) / 2;

    // 处理其余余数 i 和 24 - i 的配对情况
    for i in 1..12 {
        res += count[i] * count[24 - i];
    }
    res
}