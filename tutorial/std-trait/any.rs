// trait Any: 'static {
//     fn type_id(&self) -> TypeId;
// }

// 我们不必手动为我们的类型实现 Any trait，因为这已经被 generic blanket impl 所涵盖

// impl<T: 'static + ?Sized> Any for T {
//     fn type_id(&self) -> TypeId {
//         TypeId::of::<T>()
//     }
// }

// 通过使用 downcast_ref::<T>() 和 downcast_mut::<T>() 方法从一个 dyn Any 中拿出一个 T

mod m1 {
    use std::any::Any;

    #[derive(Default, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn inc(&mut self) {
            self.x += 1;
            self.y += 1;
        }
    }

    fn map_any(mut any: Box<dyn Any>) -> Box<dyn Any> {
        if let Some(num) = any.downcast_mut::<i32>() {
            *num += 1;
        } else if let Some(string) = any.downcast_mut::<String>() {
            *string += "!";
        } else if let Some(point) = any.downcast_mut::<Point>() {
            point.inc();
        }
        any
    }

    pub fn t1() {
        let mut vec: Vec<Box<dyn Any>> = vec![
            Box::new(0),
            Box::new(String::from("a")),
            Box::new(Point::default()),
        ];
        // vec = [0, "a", Point { x: 0, y: 0 }]
        vec = vec.into_iter().map(map_any).collect();
        // vec = [1, "a!", Point { x: 1, y: 1 }]

        if let Some(v0) = vec[0].downcast_mut::<i32>() {
            println!("v0 {}", v0);
        }
        if let Some(v1) = vec[1].downcast_mut::<String>() {
            println!("v1 {}", v1);
        }
        if let Some(v2) = vec[2].downcast_mut::<Point>() {
            println!("v2 {:?}", v2);
        }
    }
}

// 这个 trait 很少需要用到，因为在大多数情况下，参数化多态要优于临时多态性，
// 后者也可以用枚举（enum）来模拟，枚举具有更好的类型安全，需要的间接（抽象）也更少。
// 例如，我们可以用下面的方式实现上面的例子
mod m2 {
    #[derive(Default, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn inc(&mut self) {
            self.x += 1;
            self.y += 1;
        }
    }

    enum Stuff {
        Integer(i32),
        String(String),
        Point(Point),
    }

    fn map_stuff(mut stuff: Stuff) -> Stuff {
        match &mut stuff {
            Stuff::Integer(num) => *num += 1,
            Stuff::String(string) => *string += "!",
            Stuff::Point(point) => point.inc(),
        }
        stuff
    }

    pub fn t2() {
        let mut vec = vec![
            Stuff::Integer(0),
            Stuff::String(String::from("a")),
            Stuff::Point(Point::default()),
        ];
        // vec = [0, "a", Point { x: 0, y: 0 }]
        vec = vec.into_iter().map(map_stuff).collect();
        // vec = [1, "a!", Point { x: 1, y: 1 }]

        if let Stuff::Integer(v0) = vec[0] {
            println!("v0 {}", v0);
        }
        // if let Stuff::String(v1) = &vec[1]
        if let Stuff::String(ref v1) = vec[1] {
            println!("v1 {}", v1);
        }
        if let Stuff::Point(ref v2) = vec[2] {
            println!("v2 {:?}", v2);
        }
    }
}

fn main() {
    // m1::t1();
    m2::t2();
}

// 尽管Any很少被需要用到，但是在某些时候它也会十分地便利，
// 正如我们在后面错误处理（Error Handling）部分所看到的那样
