use std::collections::HashMap;

/// 给定一组单词，找到每个单词里面重复出现的字母，返回这些字母，并且相同的字母也要按照出现的次数来返回
/// 单词只包含小写字母
/// 对每个单词建立一个字母统计表，记录每个字母的出现次数。然后对比各个单词，得到共同出现的字母最小出现次数
#[test]
pub fn test_common_chars() {
    let words = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
    let mut letters = HashMap::new();
    //
    // for s in words {
    //     for c in s.chars() {
    //         *letters.entry(c).or_insert(0) += 1;
    //     }
    // }
    // println!("map: {:?}", letters)

    letters.insert("ke1", [0;11]);
    letters.insert("ke2", [0;11]);
    letters.insert("ke3", [0;11]);

    for(k, v) in letters {
        v[0].min(v[1]);
    }
    let res = common_chars(words);
    println!("res: {:?}", res)
}

fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut reqs = HashMap::new();
    for word in words {
        let mut req = [0;26];
        for letter in word.chars() {
            let index = (letter as u8 - b'a') as usize;
            req[index] += 1;
        }
        reqs.insert(word, req.clone());
    }
    println!("req: {:?}", reqs);
    let mut res = vec![];
    for i in 0..26 {
        let min = reqs.iter().map(|(k,&f)| f[i]).min().unwrap();
        for _ in 0..min {
            let i = i as u8;
            let letter = (( i + b'a') as char).to_string();
            res.push(letter.clone());
        }
    }
    res
}

// fn common_chars(words: Vec<String>) -> Vec<String> {
//     let mut reqs = vec![];
//     for word in words {
//         let mut req = [0;26];
//         for letter in word.chars() {
//             let index = (letter as u8 - b'a') as usize;
//             req[index] += 1;
//         }
//         reqs.push(req.clone());
//     }
//     println!("req: {:?}", reqs);
//     let mut res = vec![];
//     for i in 0..26 {
//         let min = reqs.iter().map(|&f| f[i]).min().unwrap();
//         for _ in 0..min {
//             let i = i as u8;
//             let letter = (( i + b'a') as char).to_string();
//             res.push(letter.clone());
//         }
//     }
//     res
// }