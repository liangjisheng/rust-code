#[derive(Debug)]
// 导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体

struct Rectangle {
    width: u32,
    height: u32,
}

// Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现
// 结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字

// 结构体 impl 块可以写几次，效果相当于它们内容的拼接

// 单元结构体 结构体可以只作为一种象征而无需任何成员
// struct UnitStruct;
// 我们称这种没有身体的结构体为单元结构体（Unit Struct）

impl Rectangle {
    // 3个结构体方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    fn heigher(&self, rect: &Rectangle) -> bool {
        self.height > rect.height
    }

    //结构体关联函数
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1's area is {}", rect1.area());

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    println!("{}", rect1.wider(&rect2));

    let rect3 = Rectangle::create(30, 50);
    println!("{:?}", rect3);
}
