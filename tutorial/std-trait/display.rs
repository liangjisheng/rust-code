// trait Display {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
// }

// Display类型可以被序列化为对用户更为友好的String类型

mod m1 {
    use std::fmt;

    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    pub fn t1() {
        println!("origin: {}", Point::default());
        // prints "origin: (0, 0)"

        // get Point's Display representation as a String
        let stringified_point = format!("{}", Point::default());
        assert_eq!("(0, 0)", stringified_point);
    }

    // 除了使用format!宏让一个类型以String类型显示，我们还可以使用ToString trait
    // trait ToString {
    //     fn to_string(&self) -> String;
    // }

    // 这个 trait 不需要我们实现，事实上，由于 generic blanket impl，我们也不能去实现它，
    // 因为所有实现了Display的类型都会自动实现 ToString
    // impl<T: Display + ?Sized> ToString for T;

    pub fn display_point() {
        let origin = Point::default();
        assert_eq!(format!("{}", origin), "(0, 0)");
    }

    pub fn point_to_string() {
        let origin = Point::default();
        assert_eq!(origin.to_string(), "(0, 0)");
    }

    pub fn display_equals_to_string() {
        let origin = Point::default();
        assert_eq!(format!("{}", origin), origin.to_string());
    }
}

fn main() {
    m1::t1();
    m1::display_point();
    m1::point_to_string();
    m1::display_equals_to_string();
}
