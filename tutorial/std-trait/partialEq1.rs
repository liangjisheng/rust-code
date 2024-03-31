// https://juejin.cn/post/7222294792893136933

mod m1 {
    struct Person {
        name: String,
        age: u8,
    }

    // 自定义比较函数

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age
        }
    }

    impl Eq for Person {}

    pub fn p1() {
        let p1 = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let p2 = Person {
            name: "Bob".to_string(),
            age: 30,
        };
        if p1 == p2 {
            println!("p1 equals p2");
        } else {
            println!("p1 does not equal p2");
        }
    }
}

mod m2 {
    #[derive(PartialEq, Eq)]
    struct Pair<T> {
        first: T,
        second: T,
    }

    pub fn p2() {
        let p1 = Pair {
            first: 1,
            second: 2,
        };
        let p2 = Pair {
            first: 2,
            second: 1,
        };
        if p1 == p2 {
            println!("p1 equals p2");
        } else {
            println!("p1 does not equal p2");
        }
    }
}

mod m3 {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u8,
    }

    pub fn p3() {
        let p1 = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let p2 = Person {
            name: "Bob".to_string(),
            age: 40,
        };
        if p1 < p2 {
            println!("p1 is younger than p2");
        } else {
            println!("p1 is older than or equal to p2");
        }
    }
}

mod m4 {
    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u8,
    }

    use std::fmt;

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({})", self.name, self.age)
        }
    }

    pub fn p4() {
        let p1 = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let p2 = Person {
            name: "Bob".to_string(),
            age: 40,
        };
        println!("p1: {}", p1);
        println!("p2: {}", p2);
    }
}

fn main() {
    // m1::p1();
    // m2::p2();
    // m3::p3();
    m4::p4();
}
