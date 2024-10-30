#[test]
pub fn test_valid_strings() {
    let res = valid_strings(3);
    println!("res: {:?}", res)
}

fn valid_strings(n: i32) -> Vec<String> {
    fn dfs(i: usize, path: &mut Vec<char>, ans: &mut Vec<String>) {
        if i == path.len() {
            ans.push(path.iter().collect());
            return;
        }

        //填1
        path[i] = '1';
        dfs(i + 1, path, ans);

        //填0
        if i == 0 || path[i - 1] == '1' {
            path[i] = '0'; //直接覆盖
            dfs(i + 1, path, ans);
        }
    }

    let mut ans = vec![];
    let mut path = vec!['\0'; n as usize];
    dfs(0, &mut path, &mut ans);
    ans
}