#[derive(Debug)] //实现debug调试打印
struct Rectangle{
    w:u32,
    h:u32,
}

// 关键字 impl：为Rectangle添加一些方法，实现一些共功能
impl Rectangle {
    fn area(&self/*self是必须传入的参数，代表变量本身*/) -> u32 {
        self.w * self.h
    }//注意：不需要加分号

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }//注意：不需要加分号
}// 不需要加分号

// 实现关联方法，在Rectangle的命名空间外实现插入方法
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { w: size, h: size }
    }
}

fn main() {
    //let rect1 = (30, 50);

    let b = Rectangle{
                    w:30,
                    h:50,
                };

    let c = Rectangle{
        w:40,
        ..b //注意rust中，只有表达式能有返回值
    };

    let sq: Rectangle = Rectangle::square(3);

    println!("{:#?}  \n\n c is \n{:#?}", b, c);

    println!(
        "The area of the rectangle is {} {} {}",
        b.area(), b.can_hold(&c), sq.area()
    );

}

// 使用元组方式实现计算长方形面积
// fn area(a: (u32, u32)) -> u32 {
//     a.0 * a.1
// }

//使用结构体实现
// fn area(a: Rectangle) -> u32 {
//     a.w * a.h
// }