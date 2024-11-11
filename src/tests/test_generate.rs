#[test]
pub fn test_generate() {
    let res = generate(5);
    println!("res: {:?}", res)
}

fn generate(num_row: i32) -> Vec<Vec<i32>> {
    let n = num_row as usize;
    let mut triangle:Vec<Vec<i32>> = Vec::new();
    // Start each row with all 1s for elements.
    for i in 0..n {
        let mut row = vec![1; i + 1];
        // Calculate values in between the first and last element of the row.
        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }
        // Push the current row to the triangle
        triangle.push(row);
    }
    triangle
}

// fn generate(num_row: i32) -> Vec<Vec<i32>> {
//     let n = num_row as usize;
//     let mut res = vec![vec![]; n];
//     for i in 0..n {
//        res[i].resize(i + 1, 1);
//         println!("outer i: {}", i);
//         for j in 1..i {
//             println!("inner i: {}", i);
//             //左上方的数 + 正上方的数
//             res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
//         }
//     }
//     res
// }