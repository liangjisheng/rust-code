# package,crate

我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。
如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，
且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 
下的文件都会被编译成一个独立的二进制 crate。

从 crate 根节点开始: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 
而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。

声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden声明了一个叫做garden的模块。
编译器会在下列路径中寻找模块代码：

内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
在文件 src/garden.rs
在文件 src/garden/mod.rs

声明子模块: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在src/garden.rs
中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：

内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
在文件 src/garden/vegetables.rs
在文件 src/garden/vegetables/mod.rs

模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate
内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的
Asparagus 类型可以在 crate::garden::vegetables::Asparagus 被找到.

src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 
模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树 (module tree).