# crate

## 包 Crate

对于 Rust 而言，包是一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库。

一个包会将相关联的功能打包在一起，使得该功能可以很方便的在多个项目中分享。例如标准库中没有提供但是
在三方库中提供的 rand 包，它提供了随机数生成的功能，我们只需要将该包通过 use rand; 引入到当前项
目的作用域中，就可以在项目中使用 rand 的功能：rand::XXX。

同一个包中不能有同名的类型，但是在不同包中就可以。例如，虽然 rand 包中，有一个 Rng 特征，可是我们
依然可以在自己的项目中定义一个 Rng，前者通过 rand::Rng 访问，后者通过 Rng 访问，对于编译器而言，
这两者的边界非常清晰，不会存在引用歧义。

create: 表示项目，是 Rust 中的独立编译单元。每个 create 对应生成一个库或可执行文件（.lib/.dll/.so/.exe）
crate 是一个二进制项(binary)或者库(library)
crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块(module)

## 项目 Package

由于 Package 就是一个项目，因此它包含有独立的 Cargo.toml 文件，以及因为功能性被组织在一起的一个
或多个包。一个 Package 只能包含一个库(library)类型的包，但是可以包含多个二进制可执行类型的包。

package 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去
构建这些 crate.

package 中可以包含至多一个库 crate(library crate)。package 中可以包含任意多个二进制 crate(binary crate)
但是必须至少包含一个 crate（无论是库的还是二进制的）

Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同

## 易混淆的 Package 和 crate

看完上面，相信大家看出来为何 Package 和包容易被混淆了吧？因为你用 cargo new 创建的 Package 和
它其中包含的包是同名的！

不过，只要你牢记 Package 是一个项目工程，而 crate 只是一个编译单元，基本上也就不会混淆这个两个概念了：
src/main.rs 和 src/lib.rs 都是编译单元，因此它们都是包。

一个真实项目中典型的 Package，会包含多个二进制包，这些包文件被放在 src/bin 目录下，每一个文件都是独立
的二进制包，同时也会包含一个库包，该包只能存在一个 src/lib.rs：

```log
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
└── simple_example.rs
```

唯一库包：src/lib.rs
默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
集成测试文件：tests 目录下
基准性能测试 benchmark 文件：benches 目录下
项目示例：examples 目录下
