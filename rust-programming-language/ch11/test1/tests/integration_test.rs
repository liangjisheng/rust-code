use test1;

// 我们仍然可以通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试。
// 也可以使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试
// cargo test --test integration_test

#[test]
fn it_adds_two() {
    assert_eq!(4, test1::add_two(2));
}
