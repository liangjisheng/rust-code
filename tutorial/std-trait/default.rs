mod m1 {
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    // 可以为实现了Default的类型构造默认值
    // 这在快速构建原型的时候十分有用，尤其是在我们没有过多要求而只需要一个类型实例的情况下
    impl Default for Color {
        // default color is black
        fn default() -> Self {
            Color { r: 0, g: 0, b: 0 }
        }
    }

    pub fn t1() {
        let c1 = Color::default();
        println!("r {}, g {}, b {}", c1.r, c1.g, c1.b);
    }

    // 当我们想要显式地把函数暴露给用户时，也可以选择这样做
    struct Canvas;
    enum Shape {
        Circle,
        Rectangle,
    }

    impl Canvas {
        // let user optionally pass a color
        fn paint(&mut self, shape: Shape, color: Option<Color>) {
            // if no color is passed use the default color
            let color = color.unwrap_or_default();
            // etc
        }
    }

    // 当我们需要构造泛型类型时，Default在泛型上下文中也是有用的
    fn guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
        for _ in 0..min_len.saturating_sub(vec.len()) {
            vec.push(T::default());
        }
        vec
    }
}

mod m2 {
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    // 可以为实现了Default的类型构造默认值
    // 这在快速构建原型的时候十分有用，尤其是在我们没有过多要求而只需要一个类型实例的情况下
    impl Default for Color {
        // default color is black
        fn default() -> Self {
            Color { r: 0, g: 0, b: 0 }
        }
    }

    // 我们还可以利用Default类型结合 Rust 的结构体更新语法（struct update syntax）
    // 来对结构体部分初始化。现在，我们有一个Color结构体构造函数new，该函数接收结构体的所有成员作为参数
    impl Color {
        fn new(r: u8, g: u8, b: u8) -> Self {
            Color { r, g, b }
        }
    }

    impl Color {
        fn red(r: u8) -> Self {
            Color {
                r,
                ..Color::default()
            }
        }
        fn green(g: u8) -> Self {
            Color {
                g,
                ..Color::default()
            }
        }
        fn blue(b: u8) -> Self {
            Color {
                b,
                ..Color::default()
            }
        }
    }
}

// 还有一个Default派生宏，通过使用它我们可以像下面这样来写 Color
// default color is still black
// because u8::default() == 0
#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    m1::t1();
}
