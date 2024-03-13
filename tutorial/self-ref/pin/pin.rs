// https://zhuanlan.zhihu.com/p/348648305

use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;

struct SelfReferential {
    self_ptr: *const Self,
}

fn p1() {
    // 使用堆分配，我们可以尝试去创建一个自引用结构体
    let mut heap_value = Box::new(SelfReferential {
        self_ptr: 0 as *const _,
    });
    let ptr = &*heap_value as *const SelfReferential;
    heap_value.self_ptr = ptr;
    println!("heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);

    // 我们看到这个堆上的值的地址和它的内部指针的地址是相等的，这意味着
    // self_ptr字段是一个有效的自引用。因为heap_value只是一个指针，
    // 移动它（比如，把它作为参数传入函数）不会改变结构体自身的值
    // 所以self_ptr在指针移动后依然是有效的

    // 但是，仍然有一种方式来破坏这个示例：我们可以摆脱Box<T>或者替换它的内容
    let stack_value = mem::replace(
        &mut *heap_value,
        SelfReferential {
            self_ptr: 0 as *const _,
        },
    );
    println!("value at: {:p}", &stack_value);
    println!("internal reference: {:p}", stack_value.self_ptr);

    // 这里我们使用mem::replace函数使用一个新的结构体实例来替换堆分配的值。
    // 这使得我们把原始的heap_value移动到栈上，而结构体的self_ptr字段现在
    // 是一个仍然指向旧的堆地址的悬垂指针。当你尝试在 playground 上运行这个
    // 示例时，你会看到打印出的"value at:"和"internal reference:"这一
    // 行确实是输出的不同的指针。因此，在堆上分配一个值并不能保证自引用的安全。

    // 出现上面的破绽的基本问题是，Box<T>允许我们获得堆分配值的&mut T引用。
    // 这个&mut引用让使用类似mem::replace或者mem::swap的方法使得堆上值
    // 失效成为可能。为了解决这个问题，我们必须阻止创建对自引用结构体的&mut引用。
}

// pinning API 以Pin包装类型和Unpin标记 trait 的形式提供了一个针对&mut T问题的解决方案。
// 这些类型背后的思想是对Pin的所有能被用来获得对 Unpin trait 上包装的值的&mut引用的方法
// （如get_mut或者deref_mut）进行管控。Unpin trait 是一个auto trait，它会为所有的类型
// 自动实现，除了显式选择退出（opt-out）的类型。通过让自引用结构体选择退出Unpin，就没有
// （安全的）办法从一个Pin<Box<T>>类型获取一个&mut T。因此，它们的内部的自引用就能保证仍是有效的。

struct SelfReferential_1 {
    self_ptr: *const Self,
    _pin: PhantomPinned,
}

// 我们通过添加一个类型为PhantomPinned的_pin字段来选择退出。这个类型是一个零大小标记类型，
// 它唯一目的就是不去实现Unpin trait。因为 auto trait 的工作方式，有一个字段不满足Unpin，
// 那么整个结构体都会选择退出Unpin

fn p2() {
    // 因为PhantomPinned是一个零大小类型，我们只需要它的类型名来初始化它
    let mut heap_value = Box::pin(SelfReferential_1 {
        self_ptr: 0 as *const _,
        _pin: PhantomPinned,
    });
    let ptr = &*heap_value as *const SelfReferential_1;
    // error
    // heap_value.self_ptr = ptr;

    // safe because modifying a field doesn't move the whole struct
    unsafe {
        let mut_ref = Pin::as_mut(&mut heap_value);
        Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
    }
    println!("heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);

    let stack_value = mem::replace(
        // error
        &mut *heap_value,
        SelfReferential_1 {
            self_ptr: 0 as *const _,
            _pin: PhantomPinned,
        },
    );
    println!("value at: {:p}", &stack_value);
    println!("internal reference: {:p}", stack_value.self_ptr);

    // 两个错误发生都是因为Pin<Box<SelfReferential>>类型没有实现DerefMut trait。
    // 这也正是我们想要的，因为DerefMut trait 将会返回一个&mut引用，这是我们想要避免的。
    // 发生这种情况是因为我们选择退出了Unpin并把Box::new改为了Box::pin

    // 现在的问题在于，编译器不仅发现了第 78 行的错误，还发现了第 67 行的错误。
    // 这会发生是因为编译器无法区分&mut引用的有效使用和无效使用。为了能够正常初始化，
    // 我们不得不使用不安全的get_unchecked_mut方法

    // 现在剩下来的唯一的错误是mem::replace上的期望错误。记住，这个操作试图把一个
    // 堆分配的值移动到栈上，这将会破坏存储在self_ptr字段上的自引用。通过选择退出
    // Unpin和使用Pin<Box<T>>，我们可以在编译期阻止这个操作，从而安全地使用自引用
    // 结构体。正如我们所见，编译器无法证明自引用的创建是安全的，因此我们需要使用
    // 一个不安全的块（block）并且确认其自身的正确性
}

fn main() {
    // p1();
    p2();
}
