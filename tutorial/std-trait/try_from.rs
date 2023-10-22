// TryFrom和TryInto是From和Into的可能会失败的版本
// trait TryFrom<T> {
//     type Error;
//     fn try_from(value: T) -> Result<Self, Self::Error>;
// }

// trait TryInto<T> {
//     type Error;
//     fn try_into(self) -> Result<T, Self::Error>;
// }

// 类似于Into，我们无法实现TryInto，因为它的实现是由 generic blanket impl提供

// impl<T, U> TryInto<U> for T
//     where
//         U: TryFrom<T>,
// {
//     type Error = U::Error;
//
//     fn try_into(self) -> Result<U, U::Error> {
//         U::try_from(self)
//     }
// }

// 假定在我们的程序上下文环境中，Point中的x和y如果值小于-1000或者大于1000没有意义。
// 下面是我们使用TryFrom重写之前的From实现来告诉用户，现在这种转换可以失败

mod m1 {
    use std::convert::From;
    use std::convert::TryFrom;
    use std::error;
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct OutOfBounds;

    impl fmt::Display for OutOfBounds {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "out of bounds")
        }
    }

    impl error::Error for OutOfBounds {}

    // 现在是可以出错的
    impl TryFrom<(i32, i32)> for Point {
        type Error = OutOfBounds;
        fn try_from((x, y): (i32, i32)) -> Result<Point, OutOfBounds> {
            if x.abs() > 1000 || y.abs() > 1000 {
                return Err(OutOfBounds);
            }
            Ok(Point { x, y })
        }
    }

    // 仍然是不会出错的
    impl From<Point> for (i32, i32) {
        fn from(Point { x, y }: Point) -> Self {
            (x, y)
        }
    }

    pub fn t1() {
        let p = match Point::try_from((1, 2)) {
            Ok(p) => p,
            Err(e) => {
                println!("error: {}", e);
                return;
            }
        };
        println!("x {}, y {}", p.x, p.y);
    }
}

mod m2 {
    use std::convert::{TryFrom, TryInto};
    use std::error;
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct OutOfBounds;

    impl fmt::Display for OutOfBounds {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "out of bounds")
        }
    }

    impl error::Error for OutOfBounds {}

    impl TryFrom<(i32, i32)> for Point {
        type Error = OutOfBounds;
        fn try_from((x, y): (i32, i32)) -> Result<Self, Self::Error> {
            if x.abs() > 1000 || y.abs() > 1000 {
                return Err(OutOfBounds);
            }
            Ok(Point { x, y })
        }
    }

    struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl<P> TryFrom<[P; 3]> for Triangle
    where
        P: TryInto<Point>,
    {
        type Error = P::Error;
        fn try_from([p1, p2, p3]: [P; 3]) -> Result<Self, Self::Error> {
            Ok(Triangle {
                p1: p1.try_into()?,
                p2: p2.try_into()?,
                p3: p3.try_into()?,
            })
        }
    }

    fn example() -> Result<Triangle, OutOfBounds> {
        let t: Triangle = [(0, 0), (1, 1), (2, 2)].try_into()?;
        Ok(t)
    }

    pub fn t2() {
        let p = match example() {
            Ok(p) => p,
            Err(e) => return,
        };
        println!("x {}, y {}", p.p1.x, p.p1.y);
    }
}

fn main() {
    // m1::t1();
    m2::t2();
}
