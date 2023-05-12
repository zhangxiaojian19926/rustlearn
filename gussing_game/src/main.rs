
use std::io;//加入指定模块
use rand::Rng;//随机产生数字
use std::cmp::Ordering;

fn main()
{
    println!("guess the number!");// 输出宏，输出指定的字符串

    //随机产生一个数字
    let select_number = rand::thread_rng().gen_range(1, 101);

    //println!("select number {}", select_number);

    // 写一个循环
    loop {
        println!("Please input your guess\n");

        let mut guess  = String::new();

        //输入这一行到gussess里，使用的guess引用出现异常的时候
        //mut是标记，标记变量是可变的，rust默认变量是不可变的
        //string使用的UTF-8格式编码
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("failed to tran");

        // 比较数字的大小，通过match表达式实现，列举出所有比较情况
        match guess.cmp(&select_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
            {
                println!("you win");
                break;
            }
        };
    }
}