#![allow(unused)]

// https://zhuanlan.zhihu.com/p/196647275
// https://github.com/celaus/rust-async-testing
// cargo add --dev tokio-test
// cargo add actix-rt

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn str_len(s: &str) -> usize {
    s.len()
}

async fn str_len_async(s: &str) -> usize {
    // do something awaitable ideally...
    s.len()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_str_len() {
        assert_eq!(str_len("test"), 4);
    }

    // 在函数之上使用了一个额外的属性，将其分配给一个运行时，运行异步测试就像运行其他的异步代码一样
    #[actix_rt::test]
    async fn test_str_len_async() {
        assert_eq!(str_len_async("test").await, 4);
    }

    // 如果你想使用一个不同的框架或者在不同于web服务器的用例中测试，
    // 还有另一种方式。tokio-test[6]提供了一个测试运行时

    // 创建像这样的一个宏(aw是await的缩写)
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_str_len_async_2() {
        assert_eq!(aw!(str_len_async("test")), 4);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
