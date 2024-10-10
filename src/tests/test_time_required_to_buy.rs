use std::cmp::min;

/// 有n 个人来排队买票，其中第0人在队伍最前方，第(n - 1) 人在队伍最后方
/// 给定一个下标从0开始的整数数组 tickets 数组长度为n 其中第 i 人想要购买的票数为 tickets[i]
/// 每个人买票恰好都用1秒，一个人一次只能买一张票，如果需要购买更多票，他必须走到队尾重新排队（瞬间发生，不计时间）
/// 如果一个人没有剩下需要买的票，那他将会离开队伍
/// 返回位于位置 k(下标从0开始) 的人完成买票需要的时间（以秒为单位）
///
#[test]
pub fn test_time_require_to_buy() {
    let target = vec![2, 3, 2];
    let k = 2;
    let res = time_require_to_buy(target, k);
    println!("res: {}", res);
}

/// 当第k个人完成买票的一刻，前后的人分别买了多少票
/// 假设第k 个人此时买了3长票，那么排在他前面的人，此时也至多买了3张票；排在他后面的人，此时至多买了2张票
/// 把tickets 简记为 t 一般当第k 个人买tk张票时

fn time_require_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k= k as usize;
    tickets.iter().enumerate().map(|(e,_)| min(tickets[e], if e <= k { tickets[k] } else { tickets[k] -1 } )).sum::<i32>()
}
