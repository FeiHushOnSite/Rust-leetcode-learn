#[test]
pub fn test_final_position_of_snake() {
    let res = final_position_of_snake(3, vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]);
    println!("res: {}", res);
}

fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut array: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        let row: Vec<i32> = (i * n..(i + 1) * n).collect();
        array.push(row);
    }
    println!("array: {:?}", array);
    for c in commands {
        if c.eq("UP") {
            x -= 1;
        }
        if c.eq("DOWN") {
            x += 1;
        }
        if c.eq("LEFT") {
            y -= 1;
        }
        if c.eq("RIGHT") {
            y += 1;
        }
    }
    println!("x: {}, y: {}", x, y);
    println!("res: {:?}", array[x][y]);
    array[x][y]
}
fn final_position_of_snake_(n: i32, commands: Vec<String>) -> i32 {
    let mut res = 0;
    commands.iter().for_each(|command| match command.as_str() {
        "UP" => res -= n,
        "DOWN" => res -= 1,
        "LEFT" => res += 1,
        "RIGHT" => res += 1,
        _ => unreachable!(),
    });
    res
}