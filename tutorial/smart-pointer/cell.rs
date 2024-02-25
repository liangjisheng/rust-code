// Cell 和 Box 功能差不多，但是它既允许多个变量共享内容，也允许在多个变量共享状态下修改内容
// Cell 实现了不可变值的内部可变性

// Rust的Cell是很方便，但是问题在于不是每个类型都可以用Cell包起来。
// Rust的文档说的明白，只有实现了copy属性的类型才能够用cell包裹，
// 一般来说，我们自己定义的struct和trait都是没有copy属性的，所以这种就只能用RefCell

// Cell 与RefCell的区别是，Cell是整个值做替换，而RefCell是引用着值，不需要将整个value做替换
// Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况

// 内部可变性, 现在虽然是多个共享引用，但是当你需要改变的时候你需要通过
// 共享的引用获取到可变的引用，这时候Rust会在运行期进行检查
// 如果存在data-race，那么程序就会panic。而你就得修bug了

use std::cell::{Cell, RefCell};

// 变量 c 并未加 mut 修饰，下面的代码一样可以对其修改。
// 这说明 Cell 的内部修改行为应该是采用了某种 unsafe 的形式进行的，
// 仅从函数调用的表面形式上看不出要修改变量的行为。这样就把修改操作
// 的检查从编译期转移到的运行期，由 Cell 的代码确定是否存在修改的冲突

fn c1() {
    let c = Cell::new(1);
    c.set(2);
    println!("{:?}", c);
    println!("{}", c.get());

    // 由于 rust 允许多个只读的引用，因此，Cell 等于偷偷开了个后门，
    // 允许多个不同的引用修改同一个变量内容
    // 事实上， a 和 b 都是指向 c 的可变引用，但是从表面语法上看
    // 并不需要 mut 关键字，因此可以绕过编译器的语法检查

    let a = &c;
    let b = &c;
    a.set(3);
    b.set(4);
    println!("{:?}", c);

    // 如果 cell 中的 value 可以 Copy 的，也可以用 get 方法把内容复制到其他变量
    // 如果未实现 Copy 特性，则 get 方法不能使用
    let d = c.get();
    println!("d {}", d);
}

#[derive(Debug)]
struct Point {
    x: Cell<i32>,
    y: i32,
}

fn c2() {
    let p = Point {
        x: Cell::new(1),
        y: 2,
    };
    let p1 = &p;
    let p2 = &p;
    p1.x.set(3);
    p2.x.set(4);

    println!("{:?}", p);
}

fn c3() {
    let x = RefCell::new(1);
    let y = &x;
    let z = &x;
    let v = x.borrow();
    *x.borrow_mut() = 2;
    *y.borrow_mut() = 3;
    *z.borrow_mut() = 4;
    println!("{:?}", x);
}

#[derive(Debug)]
struct Point1 {
    x: RefCell<i32>,
    y: i32,
}

fn c4() {
    let p = Point1 {
        x: RefCell::new(1),
        y: 2,
    };
    let p1 = &p;
    let p2 = &p;
    *p1.x.borrow_mut() = 3;
    *p2.x.borrow_mut() = 4;

    println!("{:?}", p);
}

fn main() {
    // c1();
    // c2();
    // c3();
    c4();
}

// Cell 只能包装 Copy 类型，而 RefCell 可以包装任何类型，并且 RefCell
// 可以获取其内部包装对象的引用，并在运行时检测可变引用的唯一性

// RefCell 内部维护了一个包装对象的引用计数， 当 RefCell.borrow 获取一个共享引用时
// 内部引用计数加一，当获取的引用离开作用域时，内部引用计数减一， 当 RefCell.borrow_mut
// 获取一个可变引用时，首先检测引用计数是否为 0，如果为 0，正常返回， 如果不为 0，直接
// panic，其实 RefCell.borrow 时也会做类似的检测，当已经获取了可变引用也是直接 panic
// 当然为了避免 panic，我们可以用 RefCell.try_borrow 和 RefCell.try_borrow_mut
// 来获取一个 Result 类型

// 因为 Cell 和 RefCell 两种类型都未实现 Sync trait，所以这两种包装类型只能用于单线程中
// 不能跨线程操作， 如果需要跨线程操作，就需要用到 Mutex 和 RwLock 了

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
