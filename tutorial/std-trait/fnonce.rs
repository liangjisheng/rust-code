// trait FnOnce<Args> {
//     type Output;
//     fn call_once(self, args: Args) -> Self::Output;
// }
//
// trait FnMut<Args>: FnOnce<Args> {
//     fn call_mut(&mut self, args: Args) -> Self::Output;
// }
//
// trait Fn<Args>: FnMut<Args> {
//     fn call(&self, args: Args) -> Self::Output;
// }

// 虽然存在这些 trait，但是在 stable 的 Rust 中，我们无法为自己的类型实现这些 trait。
// 我们能够创建的唯一能够实现这些 trait 的类型就是闭包。闭包根据其从环境中所捕获的内容来
// 决定它到底是实现FnOnce、FnMut还是Fn

// FnOnce闭包只能被调用一次，因为它会在执行过程中消耗掉某些值
fn f1() {
    let range = 0..10;
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10);

    // get_range_count(); // value used here after move
    // 迭代器上的.count()方法会消耗迭代器，因此它只能被调用一次。因此，我们的闭包也只能调用一次
}

// FnMut闭包可以被多次调用，并且可以修改它从环境中捕获到的变量。我们可以说
// FnMut 有副作用或者是有状态的（stateful）。下面是一个闭包的示例，通过
// 从迭代器中追踪它见到的最小值来过滤所有非升序的值
fn f2() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums
        .into_iter()
        .filter(|&n| {
            if n <= min {
                false
            } else {
                min = n;
                true
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending);
}

// FnOnce会获取它的参数的所有权并且只能被调用一次，但是FnMut仅要求获取参数的
// 可变引用并且可以被多次调用，从这一点上来讲，FnMut细化了FnOnce。
// FnMut可以被用于任何可以使用FnOnce的地方

// Fn闭包也可以被调用多次，但是它不能修改从环境中捕获的变量。我们可以说
// Fn闭包没有副作用或者无状态的（stateless）。下面是一个示例，从一个
// 迭代器中过滤出所有小于某个栈上变量的数字，该变量是它是环境中捕获到的
fn f3() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9);
}

// FnMut要求可变引用并且可以被多次调用，Fn只要求不可变引用并可以被多次调用，从这一点来讲，
// Fn细化了FnMut。Fn可以被用于任何可以使用FnMut的地方，当然也包括可以使用FnOnce的地方

// 如果一个闭包不从环境中捕获任何变量，从技术角度来讲它算不上是闭包，而只是一个被匿名声明的内联函数，
// 并且可以作为一个普通函数指针（即Fn）被使用和传递，这包括可以使用FnMut和FnOnce的地方
fn add_one(x: i32) -> i32 {
    x + 1
}

fn f4() {
    let mut fn_ptr: fn(i32) -> i32 = add_one;
    assert_eq!(fn_ptr(1), 2);

    // capture-less closure cast to fn pointer
    fn_ptr = |x| x + 1; // same as add_one
    assert_eq!(fn_ptr(1), 2);
}

fn f5() {
    let nums = vec![-1, 1, -2, 2, -3, 3];
    let absolutes: Vec<i32> = nums.into_iter().map(i32::abs).collect();
    assert_eq!(vec![1, 1, 2, 2, 3, 3], absolutes);
}

fn main() {
    // f1();
    // f2();
    // f3();
    // f4();
    f5();
}
