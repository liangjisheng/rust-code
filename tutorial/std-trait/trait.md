# trait

[std trait](https://www.zhihu.com/people/tony-folly/posts?page=1)

标记 Trait（Marker Traits）  
标记 trait 是不含 trait 项的 trait。它们的工作把实现类型“标记（mark）”为具有某种属性，否则就没有办法在类型系统中去表示

自动 Trait（Auto Trait）  
自动 Trait 是指如果一个类型的所有成员都实现了该 trait，该类型就会自动实现该 trait。“成员（member）”的含义取决于类型，例如：结构体的字段、枚举的变量、数组的元素、元组的项，等等。

所有的自动 trait 都是标记 trait，但不是所有的标记 trait 都是自动 trait。自动 trait 必须是标记 trait，所以编译器可以为它们提供一个自动的默认实现，如果它们有任何 trait 项，这就不可能实现了

如果一个类型是Sized，这意味着它的类型大小在编译期是可知的，并且可以在栈上创建一个该类型的实例

所有的泛型类型都有一个隐含的 Sized 约束

```rust
fn func<T>(t: &T) {}

// example above desugared
fn func<T: Sized>(t: &T) {}
```

因为所有的泛型类型上都有一个隐含的Sized约束，如果我们想要选择退出这个约束，我们需要使用特定的“宽松约束（relaxed bound）”语法——?Sized，该语法目前只为Sized trait 存在

```rust
// now T can be unsized
fn func<T: ?Sized>(t: &T) {}
```

所有的 trait 都有一个隐含的?Sized约束

```rust
trait Trait {}

// example above desugared
trait Trait: ?Sized {}
```
