#[test]
fn another() {
    // panic!("Make this test fail");
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(2));
    }

    // 可以向 assert!、assert_eq! 和 assert_ne! 宏传递一个可选的失败信息参数，
    // 可以在测试失败时将自定义失败信息一同打印出来
    #[test]
    fn test_add_two_1() {
        // assert_eq!(4, add_two(2), "not equal");
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 使用 ignore 属性来标记耗时的测试并排除他们
    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
