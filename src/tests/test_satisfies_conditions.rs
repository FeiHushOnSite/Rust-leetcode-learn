use crate::TestFuc;

#[test]
pub fn test_satisfies_conditions() {
    let dept = Dog;
    dept.test_fuc();
    print_sound(&dept);
    println!("{:?}", Department::test_fuc(&Department{ dep_id: 10, dep_name: "bix".to_string()}));
    // let grid: [[i32; 3]; 2] = [[1,0,2],[1,0,2]];
    let grid: Vec<Vec<i32>> = vec![vec![1], vec![2], vec![3]];
    let res = satisfies_conditions(grid);
    println!("{:?}", res);
    let grid1: Vec<Vec<i32>> = vec![vec![1], vec![2], vec![3]];
    let res1 = satisfies_conditions_func(grid1);
    println!("{:?}", res1);
}

struct Dog;

impl TestFuc for Dog {
    fn test_fuc(&self) {
        println!("Woof!")
    }
}

fn print_sound(a: &dyn TestFuc) {
    a.test_fuc();
}

struct Department {
    dep_id: u8,
    dep_name: String
}

impl TestFuc for Department {
    fn test_fuc(&self) {
        println!("department id :{}", self.dep_id);
        println!("department name :{}", self.dep_name);
        println!("This is the implementation for my_method in MyStruct");
        // todo!();
    }
}

pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..= m {
        for j in 0..=n {
            if i + 1 < m && grid[i][j] != grid[i + 1][j] {
                return false;
            }
            if j + 1 < n && grid[i][j] == grid[i][j + 1] {
                return false;
            }
        }
    }
    true
}

pub fn satisfies_conditions_func(grid: Vec<Vec<i32>>) -> bool {
    grid[0].windows(2).all(|col| col[0] != col[1])
        && grid.windows(2).all(|row| row[0] == row[1])
}
