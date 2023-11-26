// 就像 cargo run 会编译代码并运行生成的二进制文件一样，cargo test 在测试模式下
// 编译代码并运行生成的测试二进制文件。cargo test 产生的二进制文件的默认行为是并发
// 运行所有的测试，并截获测试运行过程中产生的输出，阻止它们被显示出来，使得阅读测试
// 结果相关的内容变得更容易。不过可以指定命令行参数来改变 cargo test 的默认行为

// 可以将一部分命令行参数传递给 cargo test，而将另外一部分传递给生成的测试二进制文件。
// 为了分隔这两种参数，需要先列出传递给 cargo test 的参数，接着是分隔符 --，再之后是
// 传递给测试二进制文件的参数。运行 cargo test --help 会提示 cargo test 的有关参数
// 而运行 cargo test -- --help 可以提示在分隔符之后使用的有关参数。

// 想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
// $ cargo test -- --test-threads=1

// 默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试中调用了
// println! 而测试通过了，我们将不会在终端看到 println! 的输出：只会看到说明测试通过的提示行。
// 如果测试失败了，则会看到所有标准输出和其他错误信息。

// 如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 --show-output 告诉 Rust 显示成功测试的输出。
// $ cargo test -- --show-output

// 为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去
// 寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将
// 每一个文件当作单独的 crate 来编译。

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        // let value = prints_and_returns_10(8);
        // assert_eq!(5, value);
    }

    // 我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行。例如，
    // 因为头两个测试的名称包含 add，可以通过 cargo test add 来运行这两个测试

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // 可以向 cargo test 传递任意测试的名称来只运行这个测试
    // cargo test one_hundred

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // 有时一些特定的测试执行起来是非常耗费时间的，所以在大多数运行 cargo test 的时候希望能排除它们。
    // 虽然可以通过参数列举出所有希望运行的测试来做到，也可以使用 ignore 属性来标记耗时的测试并排除它们
    // 如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored
    // 如果你希望不管是否忽略都要运行全部测试，可以运行 cargo test -- --include-ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
