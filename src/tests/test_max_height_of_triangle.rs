#[test]
pub fn test_max_height_of_triangle() {
    let red = 3;
    let blue = 8;
    let res = max_height_of_triangle(red, blue);
    println!("res: {}", res);

    // let mut odd = 2;
    // let mut even = 4;
    // let cnt = (1..).step_by(2).take_while(|x| {
    //     odd -= x;
    //     odd >= 0
    // }).count() * 2;
    // println!("cnt: {}", cnt);
}

fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    let mut res = 1;
    let mut odd = 1;
    let mut even = 0;

    loop {
        if(odd > red || even > blue) && (odd > blue || even > red) {
            break;
        } else {
            res += 1;
            if res % 2 == 1 {
              odd += res;
            } else {
                even += res;
            }
        }
    }
    res - 1
    // let mut count = 0;
    // if red <= 1 {
    //     return red;
    // }
    //
    // if red >= 1 && blue == 1 {
    //     return 2;
    // }
    //
    // if blue >= red {
    //     while blue > count && red >= 0 {
    //         count += 1;
    //         if count % 2 != 0 {
    //             blue = blue - count;
    //         } else {
    //             red = red - count;
    //         }
    //     }
    //
    // } else {
    //     while red > count && blue >= 0{
    //         count += 1;
    //         if count % 2 == 0 {
    //             red = red - count;
    //         } else {
    //             blue = blue - count;
    //         }
    //     }
    //
    // }
    // count

}
