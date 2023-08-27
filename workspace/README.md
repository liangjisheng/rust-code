# workspace

一个Package可以管理多个Crate，但是想要只使用一个Package管理一个大项目还是有所不妥  
Cargo工作空间(workspaces)的功能类似于Maven的Module，即将一个庞大的项目拆分成多个功能相对独立的Package  
比如说将项目拆分为三Package：product-management,order-management,user-management

目前使用cargo new指令无法之间创建工作空间，因此还是建议先新建一个目录，然后手动创建一个Cargo.toml文件  
然后手动加上[workspace]以指定该目录是一个工作空间

当处于工作空间所在根目录下执行cargo new创建Package时，cargo并不会直接在Cargo.toml的workspace.members  
中增加新Package的信息(即不会把新创建的包纳入工作空间管理中)，终端只会返回信息当前目录是一个工作目录，此时需要  
手动设置一下workspace.members

工作空间在顶级目录有一个 target 目录，每一个Package内没有target目录，即使进入子Package中运行cargo build  
生成的结果依然会保存在工作空间/target中，这样让所有的Package共享一个target目录，可以避免其他Package多余的重复构建

子Package之间互相依赖：默认情况下cargo不假定工作空间中的crate会互相依赖，所欲要显式声明crate之间的依赖关系  
具体做法为，比如order-management依赖同级的user-management，则在order-management/Cargo.toml中需要声明

```toml
[dependencies]

user-management = { path = "../user-management" }
```

如果要在工作空间中运行指定的二进制crate，需要增加-p参数和包名称来指定

```shell
cargo run -p product-management

# 工作空间目录下执行, 会编译所有的 package
cargo build
```

由此可见Cargo.toml可以作为Package的配置，也可以作为工作空间的配置

工作空间中使用外部依赖：

由于整个工作空间及子目录中只有根目录的一个Cargo.lock，因此工作空间根上的src/*.rs使用的依赖与  
每一个子Package的依赖的所有版本信息都交由工作空间根目录的 Cargo.lock约束

一个子Package A依赖了某个外部库a，如果子Package B或者工作空间的根没有在对应的 Cargo.toml  
中声明使用a，那么只有Package A能使用外部库a，但是约束信息还是会保存在根 Cargo.lock

如果其它Package或者根也使用外部库a，则由于Cargo.lock的存在，会强制要求工作空间中任何地方都只能用相同版本的外部库a

工作空间中测试  
运行所有测试：cargo test  
指定某个子crate测试：cargo test -p mp-core  

工作空间中发布：如果需要单独发布每一个子Package，需要进入到对应的目录中执行 cargo publish