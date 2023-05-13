use std::io;

// 递归实现斐波那契数列
fn fb(x:u32)->u32{
    match x {
        0 => 1,
        1 => 1,
        _ => fb(x - 1) + fb(x - 2)
    }
}

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input error");

    let n:u32 = input.trim().parse().expect("parse error");

    println!("the num is {}", fb(n));
}
