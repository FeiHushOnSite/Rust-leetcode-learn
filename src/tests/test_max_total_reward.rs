#[test]
pub fn test_max_total_reward() {
    let reward_values = vec![1, 1, 3, 3];
    let res = max_total_reward(reward_values);
    println!("res: {}", res)
}

fn max_total_reward(reward_values: Vec<i32>) -> i32 {
    let mut reward_value = reward_values.clone();
    reward_value.sort();
    let m = reward_values[reward_value.len() - 1] as usize;
    let mut dp = vec![0; 2 * m];
    dp[0] = 1;// 使用 dp[k] 表示总奖励 k 是否可获得，初始时 dp[0]=1

    for &x in &reward_value {
        let x = x as usize;
        println!("x: {}", x);
        for k in (x..2 * x).rev() {
            println!("k: {}", k);
            if dp[k - x] == 1 {
                dp[k] = 1;
            }
        }
    }

    let mut res = 0;
    for(i, &value) in dp.iter().enumerate() {
        if value == 1 {
            res = i;
        }
    }
    res as i32
}