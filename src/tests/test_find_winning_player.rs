use std::collections::HashMap;

/// 有 n 位玩家进行比赛， 玩家编号依次为0 到  n - 1
/// 给定一个长度n的整数数组 skills和一个正整数k，其中skills[i]是第i 位玩家的技能等级skills中所有整数互不相同
/// 所有玩家编号从0 到 n- 1排成一列
/// 比赛方式
/// 队列中最前面两名玩家进行 一场比赛，技能等级更高的玩家胜出
/// 比赛后，获胜者保持在队列前头，而失败者排到队尾
/// 这个比赛的赢家是第一位连续赢下k场比赛的玩家
#[test]
pub fn test_find_winning_player() {
     // let skills = vec![4, 2, 6, 3, 9];
    let skills = vec![18, 15, 20];

    let res = find_winning_player(skills, 2);
    println!("res: {}", res)
}
fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut win = 0;
    for (i, _) in skills.iter().enumerate().skip(1) {
        if skills[i] > skills[res] { // 打擂台，发现新的最大值
            res = i;
            win = 0;
        }
        win += 1; // 获胜回合 +1
        if win == k { // 连续赢下 k 场比赛
            break;
        }
    }
    // 如果 k 很大，那么 max_i 就是 skills 最大值的下标，毕竟最大值会一直赢下去
    win as _
}
// fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
//     let mut map = HashMap::new();
//     let mut count = 0;
//     skills.iter().enumerate().for_each(|(i, &a)| {
//         skills.iter().skip(i + 1).for_each(|&b| {
//             if a > b {
//                 map.insert(i, count);
//             }
//             count += 1;
//             println!("Comparing {} and {}", a, b);
//         })
//     });
//     println!("map: {:?}", map);
//     let max_key = map.iter().max_by_key(|&(_, value)| value).map(|(key, _)| key);
//     match max_key {
//         None => { 0 }
//         Some(key) => { *key as i32 }
//     }
// }