use core::time;
use std::{time::Duration, thread::sleep};

use rand::Rng;//随机产生数字

// T 表示泛型
struct Cacher<T> 
    where T : Fn(u32)->u32
{
    calculation: T,
    value:Option<u32>,
}

// 实现Cacher的trait
impl<T> Cacher<T>
    where T:Fn(u32)->u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher { calculation, value: None }
    }

    // 保存最开启创建实例的值
    // 保存至对应value变量中
    fn value(&mut self, arg : u32) -> u32{
        match self.value {
            // 防止当前值与上次值不同，还采用上次的值，导致返回异常
            Some(v) => {
                if self.value.unwrap() != arg{
                    self.value = Some(v);
                }
                v
            },

            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
} 

// 生命周期外部临时引用
// 可以采用option 或 result 返回局部变量
// 闭包使用
fn cal(cal: u32)-> Option<u32>{

    let mut value = Cacher::new(|num| {
                            sleep(Duration::from_micros(2));
                            num
                        });

    if cal < 20 && cal > 10 {
        value.value(100);
    } else if cal >= 30 {
        value.value(150);
    } else {
        value.value(1000);
    }

   let val =  match value.value {
                        Some(v) => v,
                        None => 0,
                    };

    Some(val) // 拷贝一个单独的值，后续使用 
}

fn main() {

    //随机产生一个数字
    let select_number = rand::thread_rng().gen_range(1, 101);

    let val = match cal(select_number) {
                        Some(val) => val,
                        None => {
                            println!("error");
                            0
                        },
                    };

    println!("Hello, world! val:{}", val);
}
