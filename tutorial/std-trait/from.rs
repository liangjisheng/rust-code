// trait From<T> {
//     fn from(T) -> Self;
// }

// trait Into<T> {
//     fn into(self) -> T;
// }

// 我们只能为自己的类型实现From<T>，因为Into<T>的实现会通过 generic blanket impl 自动提供

// impl<T, U> Into<U> for T
//     where
//         U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

// these bounds are equivalent
// T: From<i32>,
// i32: Into<T>

// 没有规则强制要求什么时候使用前者或后者，所以在每种情景下采用最合理的方式就可以了。现在让我们来看一个例子

mod m1 {
    struct Point {
        x: i32,
        y: i32,
    }

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Point { x, y }
        }
    }

    impl From<[i32; 2]> for Point {
        fn from([x, y]: [i32; 2]) -> Self {
            Point { x, y }
        }
    }

    pub fn example1() {
        // 使用 From
        let origin = Point::from((0, 0));
        let origin = Point::from([0, 0]);

        // 使用 Into
        let origin: Point = (0, 0).into();
        let origin: Point = [0, 0].into();
    }
}

// 这个实现不是对称的，因此，如果我们想要把Point转为 tuple 和 array，我们必须显式地添加下面的内容
mod m2 {
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Point { x, y }
        }
    }

    impl From<Point> for (i32, i32) {
        fn from(Point { x, y }: Point) -> Self {
            (x, y)
        }
    }

    impl From<[i32; 2]> for Point {
        fn from([x, y]: [i32; 2]) -> Self {
            Point { x, y }
        }
    }

    impl From<Point> for [i32; 2] {
        fn from(Point { x, y }: Point) -> Self {
            [x, y]
        }
    }

    pub fn example2() {
        // 从 (i32, i32) 到 Point
        let point = Point::from((0, 0));
        let point: Point = (0, 0).into();

        // 从 Point 到 (i32, i32)
        let tuple = <(i32, i32)>::from(point.clone());
        let tuple: (i32, i32) = point.into();

        // 从 [i32; 2] 到 Point
        let point = Point::from([0, 0]);
        let point: Point = [0, 0].into();

        // 从 Point 到 [i32; 2]
        let array = <[i32; 2]>::from(point.clone());
        let array: [i32; 2] = point.into();
    }
}

// From<T>的一个常见用法是精简模板代码。假定我们想要在程序中添加一个Triangle类型，
// 它里面包含三个Point，下面是我们可以构造它的方式

mod m3 {
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Point {
            Point { x, y }
        }
    }

    struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl Triangle {
        fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
            Triangle { p1, p2, p3 }
        }
    }

    impl<P> From<[P; 3]> for Triangle
    where
        P: Into<Point>,
    {
        fn from([p1, p2, p3]: [P; 3]) -> Triangle {
            Triangle {
                p1: p1.into(),
                p2: p2.into(),
                p3: p3.into(),
            }
        }
    }

    pub fn example3() {
        // 手动构造
        let triangle = Triangle {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 1, y: 1 },
            p3: Point { x: 2, y: 2 },
        };

        // 使用 Point::new
        let triangle = Triangle {
            p1: Point::new(0, 0),
            p2: Point::new(1, 1),
            p3: Point::new(2, 2),
        };

        // 使用 From<(i32, i32)> for Point
        let triangle = Triangle {
            p1: (0, 0).into(),
            p2: (1, 1).into(),
            p3: (2, 2).into(),
        };

        // 使用 Triangle::new + From<(i32, i32)> for Point
        let triangle = Triangle::new((0, 0).into(), (1, 1).into(), (2, 2).into());

        // 使用 From<[Into<Point>; 3]> for Triangle
        let triangle: Triangle = [(0, 0), (1, 1), (2, 2)].into();
    }
}

// Into<T>一个常见的用途是，使得需要拥有值的函数具有通用性，而不必关心它们是拥有值还是借用值
mod m4 {
    struct Person {
        name: String,
    }

    impl Person {
        // 接受:
        // - String
        fn new1(name: String) -> Person {
            Person { name }
        }

        // 接受:
        // - String
        // - &String
        // - &str
        // - Box<str>
        // - Cow<'_, str>
        // - char
        // 因为上面所有的类型都可以转换为 String
        fn new2<N: Into<String>>(name: N) -> Person {
            Person { name: name.into() }
        }
    }
}

fn main() {
    m1::example1();
    m2::example2();
    m3::example3();
}
