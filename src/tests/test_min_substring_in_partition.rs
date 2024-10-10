const INF: i32 = 0x3f3f3f3f;
#[test]
pub fn min_substring_in_partition() {
    let s:String = "fabccddg".to_string();
    let n = s.len();
    let mut d = vec![INF; n + 1];
    d[0] = 0;
    for i in 1..= n {
        let mut max_cnt = 0;
        let mut occ_cnt = std::collections::HashMap::new();
        for j in (1..= i).rev() {
            let c = s.as_bytes()[j - 1];
            *occ_cnt.entry(c).or_insert(0) += 1;
            max_cnt = max_cnt.max(*occ_cnt.get(&c).unwrap());
            if max_cnt * occ_cnt.len() == i - j + 1 && d[j - 1] != INF {
                d[i] = d[i].min(d[j - 1] + 1);
            }
        }
    }
    assert_eq!(d[n], 3)
}

// cargo test TARGET -- --option
// 给cargo test 的参数 <-|->分隔 给测试的二进制的参数

#[test]
pub fn rev_range() {
    for i in (1..= 10).rev() {
        println!("output value is {}", i);
    }
}
