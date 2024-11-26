/// 环形索引计算
/// 前一元素索引 (i + n - 1) %  n
/// 后一元素索引 (i + 1) % n
#[test]
pub fn test_number_of_alternating_groups() {
    let colors = vec![0, 1, 0, 0, 1];
    let res = number_of_alternating_groups(colors);
    assert_eq!(res, 3);
}

fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let n = colors.len();
    let mut res = 0;
    for i in 0..n {
        let prev = colors[(i + n - 1) % n]; //前一个元素
        let next = colors[(i + 1) % n];
        if colors[i] != prev && colors[i] != next {
            res += 1;
        }
    }
    res
}