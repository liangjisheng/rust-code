// https://course.rs/compiler/fight-with-compiler/borrowing/ref-exist-in-out-fn.html
// 本文将彻底解决一个困扰广大 Rust 用户已久的常见错误：因为在函数内外同时借用一个引用，
// 导致了重复借用错误cannot borrow *self as mutable because it is also borrowed as immutable.

// 智能指针引起的重复借用错误
// https://course.rs/compiler/fight-with-compiler/borrowing/borrow-distinct-fields-of-struct.html

struct Test {
    a: u32,
    b: u32,
}

impl Test {
    fn increase(&mut self) {
        let mut a = &mut self.a;
        let mut b = &mut self.b;
        *b += 1;
        *a += 1;
    }
    // 虽然从表面来看，a和b都可变引用了self，但是 Rust 的编译器在很多时候都足够聪明，
    // 它发现我们其实仅仅引用了同一个结构体中的不同字段，因此完全可以将其的借用权分离开来。
    // 因此，虽然我们不能同时对整个结构体进行可变引用，但是我们可以分别对结构体中的不同
    // 字段进行可变引用，当然，一个字段至多也只能存在一个可变引用，这个最基本的所有权规则
    // 还是不能违反的。变量a引用结构体字段a，变量b引用结构体字段b，从底层来说，这种方式
    // 也不会造成两个可变引用指向了同一块内存。

    fn increase1(&mut self) {
        let b = &mut self.b;
        // self.increase_a(); // second mutable borrow occurs here
        Test::increase_a_1(&mut self.a);
        *b += 1;
    }

    // 虽然increase_a在函数实现中没有访问self.b字段，但是它的签名允许它访问b，
    // 因此违背了借用规则。事实上，该函数有没有访问b不重要，因为编译器在这里只关心
    // 签名，签名存在可能性，那么就立刻报出错误。
    fn increase_a(&mut self) {
        self.a += 1;
    }

    // 此时，increase_a这个相关函数，不再使用&mut self作为签名，而是获取了结构体
    // 中的字段a，此时编译器又可以清晰的知道：函数increase_a和变量b分别引用了结构体
    // 中的不同字段，因此可以编译通过。
    fn increase_a_1(a: &mut u32) {
        *a += 1;
    }

    // 在这里，我们不再单独声明变量b，而是直接调用self.b+=1进行递增，根据借用生命周期
    // NLL的规则，第一个可变借用self.increase_a()的生命周期随着方法调用的结束而结束，
    // 那么就不会影响self.b += 1中的借用。
    fn increase2(&mut self) {
        self.increase_a();
        self.b += 1;
    }
}

fn main() {}
