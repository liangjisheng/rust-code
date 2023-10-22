mod m1 {
    use std::ops::Add;

    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    // 在 Rust 的类型系统中，对于某个类型T，T、&T、&mut T都会被视作是完全不同的类型，
    // 这意味着我们必须分别为它们提供 trait 的实现。让我们为&Point实现Add
    impl Add for &Point {
        type Output = Point;
        fn add(self, rhs: &Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    pub fn t1() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1 + p2;
        assert_eq!(p3.x, p1.x + p2.x);
        assert_eq!(p3.y, p1.y + p2.y);

        let p4 = &p1 + &p2;
        assert_eq!(p4.x, p1.x + p2.x);
        assert_eq!(p4.y, p1.y + p2.y);
    }
}

mod m2 {
    use std::ops::Add;

    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Copy, Clone)]
    struct Line {
        start: Point,
        end: Point,
    }

    impl Add for Point {
        type Output = Line;
        fn add(self, rhs: Point) -> Line {
            Line {
                start: self,
                end: rhs,
            }
        }
    }

    impl Add for &Point {
        type Output = <Point as Add>::Output;
        fn add(self, rhs: &Point) -> Self::Output {
            Point::add(*self, *rhs)
        }
    }

    pub fn t2() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let line: Line = p1 + p2;

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let line: Line = &p1 + &p2;
    }
}

mod m3 {
    use std::ops::AddAssign;

    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl AddAssign for Point {
        fn add_assign(&mut self, rhs: Point) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl AddAssign<&Point> for Point {
        fn add_assign(&mut self, rhs: &Point) {
            Point::add_assign(self, *rhs);
        }
    }

    pub fn t3() {
        let mut p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        p1 += &p2;
        p1 += p2;
        assert!(p1.x == 7 && p1.y == 10);
    }
}

fn main() {
    // m1::t1();
    // m2::t2();
    m3::t3();
}
