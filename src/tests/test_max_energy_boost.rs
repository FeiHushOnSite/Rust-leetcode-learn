/// 题目给定长度为n的数组energy_drink_a和energy_drink_b简称A，B 需要从左到右依次取n个数字
/// 每次可以从A或者B取，但如果上一次(第i - 1次)是从A取的，那么第i 次要切换必须暂停 一次，接着在第i + 1
/// 可以从B取，求可以取得的数字之和的最大值
/// 可以发现每次取数所得和只与前面一步的怎么决策有关，如果上一步切换，那么计算和的时候从上两步计算，否则从上一步计算
/// 因此我们设计d[i][0/1]分别表示第i步从A或者B取数时，可以获得最大值，这样我们可以得到相应的转移方程
/// d[i][0] = max(d[i - 1][0], d[i - 2][1] + A[i])
/// d[i][1] = max(d[i - 1][1], d[i - 2][0] + B[i])
/// 注意 i 在 1 和 2 时的特殊情况（起始下标设置为 1）。
/// 最终我们选择 max(d[n][0],d[n][1]) 作为答案。
#[test]
pub fn test_max_energy_boost() {
    let energy_drink_a = vec![1, 3, 1];
    let energy_drink_b = vec![3, 1, 1];

    let res = max_energy_boost(energy_drink_a, energy_drink_b);
    println!("res:{}", res);
}

fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let n = energy_drink_a.len();

    let mut a = energy_drink_a[0] as i64;
    let mut a_to_b = 0i64;
    let mut b_to_a = 0i64;
    let mut b = energy_drink_b[0] as i64;

    for i in 1..n {
        let tmp = a;
        a = a.max(b_to_a) + energy_drink_a[i] as i64;
        b_to_a = b;
        b = b.max(a_to_b) + energy_drink_b[i] as i64;
        a_to_b = tmp;
    }

    a.max(b)
}

// fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
//     let n = energy_drink_a.len();
//     let mut d = vec![vec![0; 2]; n + 1];
//
//     for i in 1..=n {
//         d[i][0] = d[i - 1][0] + energy_drink_a[i - 1] as i64;
//         d[i][1] = d[i - 1][1] + energy_drink_b[i - 1] as i64;
//
//         if i >= 2 {
//             d[i][0] = d[i][0].max(d[i - 2][1] + energy_drink_a[i - 1] as i64);
//             d[i][1] = d[i][1].max(d[i - 2][0] + energy_drink_b[i - 1] as i64);
//         }
//     }
//     d[n][0].max(d[n][1])
// }