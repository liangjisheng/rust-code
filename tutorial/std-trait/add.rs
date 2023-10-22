// 泛型类型和关联类型都把在 trait 的函数和方法中使用哪种具体类型的决定权交给了实现者，
// 因此这部分内容要去解释什么时候使用泛型类型，什么时候使用关联类型。

// 通常的经验法则是：
// 当每个类型只应该有 trait 的一个实现时，使用关联类型。
// 当每个类型可能会有 trait 的多个实现时，使用泛型类型。

// 比如说我们想要定义一个名为Add的 trait，该 trait 允许我们对值进行相加。
// 下面是一个最初的设计和实现，里面只使用了关联类型

mod m1 {
    trait Add {
        type Rhs;
        type Output;
        fn add(self, rhs: Self::Rhs) -> Self::Output;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Rhs = Point;
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    pub fn t1() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = p1.add(p2);
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
    }
}

// 假设现在我们想要添加这样一种功能：把i32加到Point上，其中Point里面的成员x和y都会加上i32
// 因为Add trait 没有被任何的泛型类型参数化，我们只能在每个类型上实现这个 trait 一次，这意味着，
// 我们只能一次把Rhs和Output类型都选取好！为了能够使Point和i32类型都能和Point相加，我们必须把
// Rhs从一个关联类型重构为泛型类型，这样就能够让我们根据Rhs不同的类型参数来为Point实现 trait 多次
mod m2 {
    trait Add<Rhs> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Add<Point> for Point {
        type Output = Self;
        fn add(self, rhs: Point) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<i32> for Point {
        type Output = Self;
        fn add(self, rhs: i32) -> Self::Output {
            Point {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }

    pub fn t2() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = p1.add(p2);
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);

        let p1 = Point { x: 1, y: 1 };
        let int2 = 2;
        let p3 = p1.add(int2);
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
    }
}

// 假如说我们增加了一个名为Line的新类型，它包含两个Point，现在，在我们的程序中存在
// 这样一种上下文环境，即将两个Point相加之后应该产生一个Line而不是另一个Point。
// 这在当我们当前的Add trait 设计中是不可行的，因为Output是一个关联类型，但是我们
// 通过把Output从关联类型重构为泛型类型来实现这个新需求
mod m3 {
    trait Add<Rhs, Output> {
        fn add(self, rhs: Rhs) -> Output;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Add<Point, Point> for Point {
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<i32, Point> for Point {
        fn add(self, rhs: i32) -> Point {
            Point {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }

    struct Line {
        start: Point,
        end: Point,
    }

    impl Add<Point, Line> for Point {
        fn add(self, rhs: Point) -> Line {
            Line {
                start: self,
                end: rhs,
            }
        }
    }

    pub fn t3() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3: Point = p1.add(p2);
        assert!(p3.x == 3 && p3.y == 3);

        let p1 = Point { x: 1, y: 1 };
        let int2 = 2;
        let p3 = p1.add(int2);
        assert!(p3.x == 3 && p3.y == 3);

        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let l: Line = p1.add(p2);
        assert!(l.start.x == 1 && l.start.y == 1 && l.end.x == 2 && l.end.y == 2)
    }
}

fn main() {
    // m1::t1();
    // m2::t2();
    m3::t3();
}
