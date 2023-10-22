// Trait 可以为它们的函数和方法提供默认实现

mod m1 {
    use std::os::unix::raw::off_t;

    trait Trait {
        fn method(&self) {
            println!("default impl");
        }
    }

    struct SomeType;
    struct OtherType;

    // use default impl for Trait::method
    impl Trait for SomeType {}

    impl Trait for OtherType {
        // use our own impl for Trait::method
        fn method(&self) {
            println!("OtherType impl");
        }
    }

    pub fn t1() {
        SomeType.method(); // prints "default impl"
        OtherType.method(); // prints "OtherType impl"
    }
}

// 如果 trait 中的某些方法是完全通过 trait 的另一些方法来实现的，这就非常方便了
mod m2 {
    trait Greet {
        fn greet(&self, name: &str) -> String;
        fn greet_loudly(&self, name: &str) -> String {
            self.greet(name) + "!"
        }
    }

    struct Hello;
    struct Hola;

    impl Greet for Hello {
        fn greet(&self, name: &str) -> String {
            format!("Hello {}", name)
        }
        // use default impl for greet_loudly
    }

    impl Greet for Hola {
        fn greet(&self, name: &str) -> String {
            format!("Hola {}", name)
        }
        // override default impl
        fn greet_loudly(&self, name: &str) -> String {
            let mut greeting = self.greet(name);
            greeting.insert_str(0, "¡");
            greeting + "!"
        }
    }

    pub fn t2() {
        println!("{}", Hello.greet("John")); // prints "Hello John"
        println!("{}", Hello.greet_loudly("John")); // prints "Hello John!"
        println!("{}", Hola.greet("John")); // prints "Hola John"
        println!("{}", Hola.greet_loudly("John")); // prints "¡Hola John!"
    }
}

fn main() {
    // m1::t1();
    m2::t2();
}

// 标准库中的很多 trait 为很多它们的方法提供了默认实现
