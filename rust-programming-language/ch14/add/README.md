# workspace

还需注意的是工作空间只在根目录有一个 Cargo.lock，而不是在每一个 crate 目录都有 Cargo.lock。这确保了所有的
crate 都使用完全相同版本的依赖。如果在 Cargo.toml 和 add_one/Cargo.toml 中都增加 rand crate，则 Cargo
会将其都解析为同一版本并记录到唯一的 Cargo.lock 中。使得工作空间中的所有 crate 都使用相同的依赖意味着其中的
crate 都是相互兼容的。

Cargo 确保了工作空间中任何使用 rand 的 crate 都采用相同的版本，这节省了空间并确保了工作空间中的 crate 将是相互兼容的。

为了在顶层 add 目录运行二进制 crate，可以通过 -p 参数和包名称来运行 cargo run 指定工作空间中我们希望使用的包

```shell
$ cargo run -p adder
```

也可以选择运行工作空间中特定 crate 的测试，通过在根目录使用 -p 参数并指定希望测试的 crate 名称

```shell
$ cargo test -p add_one
```

如果你选择向 crates.io发布工作空间中的 crate，每一个工作空间中的 crate 需要单独发布。就像 cargo test 一样，
可以通过 -p 参数并指定期望发布的 crate 名来发布工作空间中的某个特定的 crate。