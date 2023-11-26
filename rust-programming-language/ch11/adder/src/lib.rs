pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting1(name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn new1(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 也可以使用 Result<T, E> 编写测试
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解。
    // 为了断言一个操作返回 Err 成员，不要使用对 Result<T, E> 值使用
    // 问号表达式（?）。而是使用 assert!(value.is_err())。

    #[test]
    fn it_works1() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // 可以向 assert!、assert_eq! 和 assert_ne! 宏传递一个可选的失败信息参数，
    // 可以在测试失败时将自定义失败信息一同打印出来。任何在 assert! 的一个必需参数和
    // assert_eq! 和 assert_ne! 的两个必需参数之后指定的参数都会传递给 format! 宏

    #[test]
    fn greeting1_contains_name() {
        let result = greeting1("Carol");
        // 结果仅仅告诉了我们断言失败了和失败的行号。一个更有用的失败信息应该打印出 greeting 函数的值
        // assert!(result.contains("Carol"));

        // 可以在测试输出中看到所取得的确切的值，这会帮助我们理解真正发生了什么，而不是期望发生什么。
        // assert!(
        //     result.contains("Carol"),
        //     "Greeting did not contain name, value was `{}`",
        //     result
        // );
    }

    // 除了检查返回值之外，检查代码是否按照期望处理错误也是很重要的
    // 可以通过对函数增加另一个属性 should_panic 来实现这些。这个属性在函数中的代码
    // panic 时会通过，而在其中的代码没有 panic 时失败。
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 然而 should_panic 测试结果可能会非常含糊不清。should_panic 甚至在一些不是
    // 我们期望的原因而导致 panic 时也会通过。为了使 should_panic 测试结果更精确，
    // 我们可以给 should_panic 属性增加一个可选的 expected 参数。测试工具会确保
    // 错误信息中包含其提供的文本。
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_1001() {
        Guess::new1(200);
    }
}
