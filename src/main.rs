use std::collections::HashMap;

mod tests;
mod singleton;
mod learn_traits;

fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(7);
    v.push(11);
    v.push(15);
    let sum = two_sum(v, 9);
    println!("{:?}", sum);
    let max_num = find_maximum_number(9, 1);
    println!("{:?}", max_num);
    // let ii = min_substring_in_partition("fabccddg".to_string());
    // println!("{ii}")
}

// 给一个整数k和一个整数x，整数num的价值事它的二进制表示中在x, 2x, 3x 等位置处设置位的数目(从最低有效位开始)
// 设置位 -> set bit 指在某数的二进制表示中值为 1 的二进制位。
// x   num    Binary Representation     Price
// 1   13     000001101                 3
// 2   13     000001101                 1
// 2   233    011101001                 3
// 3   13     000001101                 1
// 3   362    101101010                 2
// -----------------------------------------------------------------------------------------
// 例子
// x   num    Binary Representation     Price     Accumulated Price
// 1   1      001                       1         1
// 1   2      010                       1         2
// 1   3      011                       2         4
// 1   4      100                       1         5
// 1   5      101                       2         7
// 1   6      110                       2         9
// 1   7      111                       3         12
pub fn find_maximum_number(k: i64, x: i32) -> i64 {
    let (mut l, mut r) = (1i64, (k + 1) << x);
    while l < r {
        let m = (l + r + 1) / 2;
        if self::accumulated_price(x, m) > k {
            r = m - 1;
        } else {
            l = m
        }
    }
    return l;
}

pub trait TestFuc {
    fn test_fuc(&self);
}

fn accumulated_bit_price(x: i32, num: i64) -> i64 {
    let period = 1i64 << x;
    let mut res = period / 2 * (num / period);
    if num % period >= period / 2 {
        res += num % period - (period / 2 - 1);
    }
    return res;
}

fn accumulated_price(x: i32, num: i64) -> i64 {
    let mut res = 0i64;
    let length = 64 - num.leading_zeros();
    for i in (x..=length as i32).step_by(x as usize) {
        res += accumulated_bit_price(i, num)
    }
    return res;
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx = HashMap::new(); // 创建一个空哈希表
    for (j, &x) in nums.iter().enumerate() { // 枚举 j
        // 在左边找 nums[i]，满足 nums[i]+x=target
        if let Some(&i) = idx.get(&(target - x)) { // 找到了
            return vec![i as i32, j as i32]; // 返回两个数的下标
        }
        idx.insert(x, j); // 保存 nums[j] 和 j
    }
    unreachable!() // 题目保证有解，循环中一定会 return
}

#[test]
pub fn test_owner_func() {
    let i = 42;
    let mut s = String::from("hi");
    // test_cal(i, &mut s);
    println!("{}", i);
    let s = test_cal(i, &mut s);
    println!("{:?}", *s);
    let mut foo = User::new();
    println!("{}", foo.email);
    foo.email = String::from("changed@bar.com");
    println!("{}", foo.email);
    println!("{}", foo.age);
}

// fn build_user() -> User {
//     User {
//         email: String::from("foo@bar.com"),
//         age: 9
//     }
// }
#[allow(dead_code)]
impl User {
    fn new() -> User {
        User {
            email: String::from("foo@bar.com"),
            age: 9,
        }
    }
}

struct User {
    email: String,
    age: i64,
}

pub fn test_cal(i: i32, s: &mut String) -> &mut String {
    let _ir = i;
    let _s2 = s.clone();
    s.push_str("world");
    return s;
}











