use std::io;

//函数参数+返回值
fn c_to_f(temp:f64) -> f64 {
    9.0 / 5.0 * temp + 32 as f64
}

//函数参数+返回值
fn f_to_c(temp:f64) -> f64 {
    (temp - 32 as f64) * 5 as f64 / 9 as f64
}

fn main() {

    let temp =  loop {
        
        // 循环输入数字，转换成对应的
        println!("please input a num");

        // 定义输入变量
        let mut te = String::new();

        io::stdin().read_line(&mut te).expect("failed to input string");

        let te:f64 =match te.trim().parse() {
            Ok(x)=>{
                println!("the input right {}", x);
                x
            },

            Err(x)=>{
                println!("input is not vaild!! please input again {}", x);
                continue;
            }
        }; 

        break te
    };
            
    println!("please input select");
    
    let select: char = loop {
        // 定义输入变量
        let mut tem = String::new();

        io::stdin().read_line(&mut tem).expect("failed to input string");   

        let seletor:char = match tem.trim().parse() {
            Ok(y) => y,
            Err(_) => {
                println!("input error");
                continue;
            }
        };

        // 匹配选择
        match seletor {
            'f' | 'c' | 'F' | 'C' => break seletor,
            _ => {//输入错误则继续运行
                println!("input error");
                continue;
            }
        }
    };

    println!("input select {}", select);

    //提供了方便的匹配方法
    match select {
        'f' | 'F' => println!("c tran to f {}", c_to_f(temp)),
        'c' | 'C' => println!("f tran to c {}",f_to_c(temp)),
        _ => println!("error input")
    };
}
