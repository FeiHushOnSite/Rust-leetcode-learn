use std::collections::HashMap;

/// 两个map做映射表，判断是否一致
#[test]
pub fn test_is_isomorphic() {
    // let arr1 = vec![1, 2, 3];
    // let arr2 = vec![3, 4, 5];
    //
    // let arr3 = arr1.iter().zip(arr2);
    // for x in arr3 {
    //     println!("arr3: {:?}", x)
    // }
    let s = "foo".to_string();
    let t = "bar".to_string();

    let res = is_isomorphic(s, t);
    println!("res: {}", res);
}

/// tips: how is rust of iter zip use
fn is_isomorphic(s: String, t: String) -> bool {
    let mut map1 = HashMap::with_capacity(26);
    let mut map2 = HashMap::with_capacity(26);

    for x in s.chars().zip(t.chars()) {
        let insert1 = *map1.entry(x.0).or_insert(x.1);
        let insert2 = *map2.entry(x.1).or_insert(x.0);
        println!("insert1: {:?}, x.1: {:?}", insert1, x.0);
        println!("insert2: {:?}, x.0: {:?}", insert2, x.1);
        if *map1.entry(x.0).or_insert(x.1) != x.1 || *map2.entry(x.1).or_insert(x.0) != x.0 {
            println!("map1: {:?}", map1);
            println!("map2: {:?}", map2);
            return false;
        }
    }
    true
}


