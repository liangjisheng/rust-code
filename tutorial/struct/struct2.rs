// impl 关键字最重要的作用，就是定义上面我们所说的 方法的属主
// 所有被 impl My_struct 块包含的代码，都只属于 My_struct 这个结构

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        // 在方法内部，可以使用点号 `self.` 来访问当前结构体的元素
        // use the . operator to fetch the value of a field via the self keyword
        self.width * self.height
    }

    // Rust 中的结构体还可以有静态方法
    // 静态方法可以直接通过结构体名调用而无需先实例化
    // 结构体的静态方法定义方式和普通方法类似，唯一的不同点是 不需要使用 &self 作为参数
    fn get_instance(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // 在 Rust 中，允许方法名跟结构体的字段名相同
    fn width(&self) -> u32 {
        self.width
    }
}

// Rust 允许我们为一个结构体定义多个 impl 块，目的是提供更多的灵活性和代码组织性

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // 创建 Rectangle 结构体的一个实例
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    //计算并输出结构体的面积
    println!(
        "width is {} height is {} area of Rectangle is {}",
        small.width,
        small.height,
        small.area()
    );

    let r1 = Rectangle::get_instance(3, 4);
    println!("area: {}", r1.area());
    println!("rs.width: {}", r1.width());

    let _r2 = Rectangle::new(1, 2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
