#[test]
pub fn test_check_two_chess_board() {
    // let mut chess_board = HashMap::new();
    // let letter = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
    // for i in 0.. letter.len() {
    //     for j in 1..= 8 {
    //         chess_board.insert(letter[i].to_string() + &j.to_string(), if (i + j) % 2 == 0 { "w" } else { "b" });
    //     }
    // }
    // let s1 = chess_board.get("a1");
    // let s2 = chess_board.get("h3");
    // println!("res: {}", s1 == s2)
    let c1: u8 = String::from("a3").to_string().as_bytes().iter().sum();
    let c2: u8 = String::from("c3").to_string().as_bytes().iter().sum();
    println!("res: {}", (c1 & 1) == (c2 & 1));
}
