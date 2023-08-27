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

// should_panic属性
// 在测试函数上，#[test]后添加#[should_panic]，表示该测试函数只有发生panic!才算执行成功。可用于检查panic功能是否正常。
// 很多时候并不是说发生任何panic都算是执行成功，只有带有指定信息的panic才算是执行成功，因此可以指定测试函数抛出的错误信息

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// 运行所有名字包含one_hundred的测试函数
// cargo test one_hundred

// 运行所有名字包含add_的测试函数
// cargo test add_

// 在测试函数上加入#[ignore]属性，可在一般运行cargo test时跳过这些函数(通常可用于修饰消耗资源较高的测试函数)
// 运行被忽略的测试：cargo test -- --ignored

// cfg标注：表示configuration(配置)，cfg标注的作用是告诉Rust编译器
// 被标注的条目只有在指定的配置选项下才被包含。
// 比如#[cfg(test)中test选项由Rust提供，用来编译和运行测试。只有cargo test
// 才会编译代码，包括模块中的helper函数和#[test]标注的函数
