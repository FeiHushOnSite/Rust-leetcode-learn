#[test]
pub fn test_dest_city() {
    let paths: Vec<Vec<String>> = vec![vec!["London".to_string(),"New York".to_string()], vec!["New York".to_string(),"Lima".to_string()], vec!["Lima".to_string(),"Sao Paulo".to_string()]];
    let res = dest_city(paths);
    println!("res : {:?}", res);
    assert_eq!(res, "Sao Paulo")
}

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let cities_a:Vec<String> = paths.iter().map(|e| e.get(0).unwrap().to_string()).collect();
    for path in &paths {
        if !cities_a.contains(&path[1]) {
            return path[1].clone();
        }
    }
    return "".to_string();
}
