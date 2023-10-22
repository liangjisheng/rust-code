// trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }

// FromStr 类型允许执行一个从&str到Self的可失败的转换。最常见的使用是在&str上调用.parse()方法

mod m1 {
    use std::str::FromStr;

    fn example<T: FromStr>(s: &'static str) {
        // 这些都是相等的
        let t: Result<T, _> = FromStr::from_str(s);
        let t = T::from_str(s);
        let t: Result<T, _> = s.parse();
        let t = s.parse::<T>(); // 最常见的
    }
}

mod m2 {
    use std::convert::TryFrom;
    use std::error;
    use std::fmt;
    use std::iter::Enumerate;
    use std::num::ParseIntError;
    use std::str::{Chars, FromStr};

    #[derive(Debug, Eq, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    #[derive(Debug, PartialEq)]
    struct ParsePointError;

    impl fmt::Display for ParsePointError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "failed to parse point")
        }
    }

    impl From<ParseIntError> for ParsePointError {
        fn from(_e: ParseIntError) -> Self {
            ParsePointError
        }
    }

    impl error::Error for ParsePointError {}

    impl FromStr for Point {
        type Err = ParsePointError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let is_num = |(_, c): &(usize, char)| matches!(c, '0'..='9' | '-');
            let isnt_num = |t: &(_, _)| !is_num(t);

            let get_num =
                |char_idxs: &mut Enumerate<Chars<'_>>| -> Result<(usize, usize), ParsePointError> {
                    let (start, _) = char_idxs
                        .skip_while(isnt_num)
                        .next()
                        .ok_or(ParsePointError)?;
                    let (end, _) = char_idxs.skip_while(is_num).next().ok_or(ParsePointError)?;
                    Ok((start, end))
                };

            let mut char_idxs = s.chars().enumerate();
            let (x_start, x_end) = get_num(&mut char_idxs)?;
            let (y_start, y_end) = get_num(&mut char_idxs)?;

            let x = s[x_start..x_end].parse::<i32>()?;
            let y = s[y_start..y_end].parse::<i32>()?;

            Ok(Point { x, y })
        }
    }

    pub fn pos_x_y() {
        let p = "(4, 5)".parse::<Point>();
        assert_eq!(p, Ok(Point::new(4, 5)));
    }

    pub fn neg_x_y() {
        let p = "(-6, -2)".parse::<Point>();
        assert_eq!(p, Ok(Point::new(-6, -2)));
    }

    pub fn not_a_point() {
        let p = "not a point".parse::<Point>();
        assert_eq!(p, Err(ParsePointError));
    }

    // FromStr和TryFrom<&str>有着相同的签名。只要我们通过其中一个实现另一个，
    // 先实现哪个并不重要。下面是对Point实现TryFrom<&str>，假定它已经实现了FromStr:
    impl TryFrom<&str> for Point {
        type Error = <Point as FromStr>::Err;
        fn try_from(s: &str) -> Result<Point, Self::Error> {
            <Point as FromStr>::from_str(s)
        }
    }
}

fn main() {
    m2::pos_x_y();
    m2::neg_x_y();
    m2::not_a_point();
}
