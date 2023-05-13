use std::io;

// 递归实现斐波那契数列
//方式1 以match方式实现
// fn fb(x:u32)->u32{
//     match x {
//         0 => 1,
//         1 => 1,
//         _ => fb(x - 1) + fb(x - 2)
//     }
// }

// 方式2 普通递归实现
// fn fb(x:u32)->u32 {
//     return if x < 2 { 1 } else {fb(x -1) + fb(x - 2)};
// }

//方式3：不通过递归实现
fn fb(x:u32)->u32{
    // f2 = f1 + f0
    // f3 = f2 + f1
    // f4 = f3 + f2 = f2 + f1 + f2

    let mut p1 = 0;
    let mut p2 = 1;
    let mut now = 1;

    // while x > 0 {
    //     now = p1 + p2;
    //     p1 = p2;
    //     p2 = now;
    //     x -= 1;
    // }

    for _i in 0..x {
        now = p1 + p2;
        p1 = p2;
        p2 = now;
    }

    return now;
}

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input error");

    let n:u32 = input.trim().parse().expect("parse error");

    println!("the num is {}", fb(n));
}
