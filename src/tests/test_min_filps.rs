#[test]
pub fn test_min_filps() {
    let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
    let res = min_filps(grid);
    println!("res: {}", res)
}

fn min_filps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut col = 0;
    let mut row = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != grid[i][m - 1 - j] { // 每列有两个不同的就+1
                col = col + 1;
            }
            if grid[i][j] != grid[n - 1 - i][j] { // 每行有两个不同的就+1
                row = row + 1;
            }
        }
    }
    col.min(row) / 2
}