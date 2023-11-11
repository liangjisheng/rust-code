#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 所有在 impl 块中定义的函数被称为 关联函数（associated functions），
    // 因为它们与 impl 后面命名的类型相关。我们可以定义不以 self 为第一参数的关联函数
    // （因此不是方法），因为它们并不作用于一个结构体的实例
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。dbg! 宏接收一个表达式的所有权
// （与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，
// 以及该表达式的结果值，并返回该值的所有权

// 我们可以把 dbg! 放在表达式 30 * scale 周围，因为 dbg! 返回表达式的值的所有权，
// 所以 width 字段将获得相同的值，就像我们在那里没有 dbg! 调用一样。我们不希望 dbg! 拥有
// rect1 的所有权，所以我们在下一次调用 dbg! 时传递一个引用

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!("rect1 is {:?}", sq);
}
