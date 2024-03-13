# test

cfg标注：表示configuration(配置)，cfg标注的作用是告诉Rust编译器
被标注的条目只有在指定的配置选项下才被包含。
比如#[cfg(test)中test选项由Rust提供，用来编译和运行测试。只有cargo test
才会编译代码，包括模块中的helper函数和#[test]标注的函数

就像 cargo run 会编译代码并运行生成的二进制文件一样，cargo test 在测试模式下
编译代码并运行生成的测试二进制文件。cargo test 产生的二进制文件的默认行为是并发
运行所有的测试，并截获测试运行过程中产生的输出，阻止它们被显示出来，使得阅读测试
结果相关的内容变得更容易。不过可以指定命令行参数来改变 cargo test 的默认行为

如果你只想生成编译生成文件，不想看 cargo test 的输出结果，还可以使用 
cargo test --no-run

可以将一部分命令行参数传递给 cargo test，而将另外一部分传递给生成的测试二进制文件。
为了分隔这两种参数，需要先列出传递给 cargo test 的参数，接着是分隔符 --，再之后是
传递给测试二进制文件的参数。运行 cargo test --help 会提示 cargo test 的有关参数
而运行 cargo test -- --help 可以提示在分隔符之后使用的有关参数。

想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
$ cargo test -- --test-threads=1

默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试中调用了
println! 而测试通过了，我们将不会在终端看到 println! 的输出：只会看到说明测试通过的提示行。
如果测试失败了，则会看到所有标准输出和其他错误信息。

如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 --show-output 告诉 Rust 显示成功测试的输出。
$ cargo test -- --show-output

运行所有名字包含 one_hundred 的测试函数
cargo test one_hundred

运行所有名字包含 add_ 的测试函数
cargo test add_

在测试函数上加入#[ignore]属性，可在一般运行cargo test时跳过这些函数
(通常可用于修饰消耗资源较高的测试函数)
运行被忽略的测试：cargo test -- --ignored

运行 tests 模块中的被忽略的测试函数
cargo test tests -- --ignored

运行名称中带 run 且被忽略的测试函数
cargo test run -- --ignored

为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去
寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将
每一个文件当作单独的 crate 来编译。

# test

```shell
# 传递所希望运行的测试名称的参数来选择运行哪些测试
cargo test test_add_two_1
#指定部分测试的名称
cargo test it
```

当使用 cargo test 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了
test 属性的函数，并报告每一个测试是通过还是失败

只要有一个函数是 FAILED ，则整个函数的测试结果是 FAILED

当运行多个测试时， Rust 默认使用线程来并行运行。这意味着测试会更快地运行完毕，
所以你可以更快的得到代码能否工作的反馈。因为测试是在同时运行的，你应该确保测试
不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量

如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 
--test-threads 参数和希望使用线程的数量给测试二进制文件

```shell
$ cargo test -- --test-threads=1
```

默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试
中调用了 println! 而测试通过了，我们将不会在终端看到 println! 的输出：只会看到
说明测试通过的提示行。如果测试失败了，则会看到所有标准输出和其他错误信息。

如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 --show-output 告诉 
Rust 显示成功测试的输出

```shell
$ cargo test -- --show-output
```

#如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored

如果你希望不管是否忽略都要运行全部测试，可以运行 cargo test -- --include-ignored

单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的  
tests 模块，并使用 cfg(test) 标注模块。测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test  
时才编译和运行测试代码，而在运行 cargo build 时不这么做，cfg 属性代表 configuration ，它告诉  
Rust 其之后的项只应该被包含进特定配置选项中。在这个例子中，配置选项是 test

为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去寻找这个目录中  
的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译
我们需要在文件顶部添加 use adder。这是因为每一个 tests 目录中的测试文件都是完全独立的 crate  
所以需要在每一个文件中导入库。tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行  
cargo test 时编译这个目录中的文件

也可以使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试

```shell
# 运行 tests 目录下的 test1.rs 集成测试文件
cargo test --test test1
cargo test --test integration_test
```

如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests  
目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。只有库 crate 才会向其他  
crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行

