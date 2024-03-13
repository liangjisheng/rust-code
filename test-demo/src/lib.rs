pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
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

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // 自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("alice");
        // let target = "bob";
        let target = "alice";
        assert!(
            result.contains(target),
            "你的问候中并没有包含目标姓名 {} ，你的问候是 `{}`",
            target,
            result
        );
    }

    // should_panic属性
    // 在测试函数上，#[test]后添加#[should_panic]，表示该测试函数只有发生
    // panic!才算执行成功。可用于检查panic功能是否正常。
    // 很多时候并不是说发生任何panic都算是执行成功，只有带有指定信息的panic
    // 才算是执行成功，因此可以指定测试函数抛出的错误信息

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_1() {
        Guess::new(200);
    }

    // 如果注意看，你会发现 expected 的字符串和实际 panic 的字符串可以不同，
    // 前者只需要是后者的字符串前缀即可，如果改成
    // #[should_panic(expected = "Guess value must be less than")]
    // 一样可以通过测试。

    // 使用 ignore 属性来标记耗时的测试并排除他们
    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // 大家注意看，I got the value 4 并没有被输出，因为该测试顺利通过了，
    // 如果就是想要看所有的输出，该怎么办呢？
    // cargo test -- --show-output

    #[test]
    fn this_test_will_fail() {
        // let value = prints_and_returns_10(8);
        // assert_eq!(5, value);
    }

    use pretty_assertions::assert_eq; // 该包仅能用于测试

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    // debug_assert!, debug_assert_eq!, debug_assert_ne! 这三个在功能
    // 上与之前讲解的版本并无区别，主要区别在于，debug_assert! 系列只能在 Debug
    // 模式下输出
    #[test]
    fn test_debug() {
        let a = 4;
        let b = 1 + 3;
        debug_assert_eq!(
            a, b,
            "我们在测试两个数之和{} + {}，这是额外的错误信息",
            a, b
        );
    }
}
