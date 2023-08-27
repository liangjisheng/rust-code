use std::ops::Add;

fn double<T>(i: T) -> T
where
    T: Add<Output = T> + Clone + Copy,
{
    i + i
}

// 上面的字母T就是泛型(和变量x的含义是相同的)，它用来代表各种可能的数据类型。
// 多数时候泛型使用单个大写字母来表示，但也可以使用多个字母来表示

// 函数名称后面的<T>表示在函数作用域内定义一个泛型T，这个泛型只能在函数签名和函数体内使用，
// 就跟在一个作用域内定义一个变量，这个变量只能在该作用域内使用是一样的。而且，泛型本就是代表各种数据类型的变量

// 在double的函数体内需要对泛型T的值i进行加法操作，但只有实现了std::ops::Add Trait
// 的类型才能使用+进行加法操作。因此要限制泛型T是那些实现了std::ops::Add的数据类型

// 限制泛型也叫做Trait绑定(Trait Bound)，其语法有两种
// 在定义泛型类型T时，使用类似于T: Trait_Name这种语法进行限制
// 在返回值后面、大括号前面使用where关键字，如where T: Trait_Name

// 下面两种写法是等价的
// fn f<T: Clone + Copy>(i: T) -> T{}

// fn f<T>(i: T) -> T
//     where T: Clone + Copy {}

// 更复杂的示例：
// fn query<M: Mapper + Serialize, R: Reducer + Serialize>(
//     data: &DataSet, map: M, reduce: R) -> Results
// {
//     ...
// }

// 此时，下面写法更友好、可读性更高
// fn query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
//     where M: Mapper + Serialize,
//           R: Reducer + Serialize
// {
//     ...
// }

// 观察指定变量数据类型的写法i: i32和限制泛型的写法T: Trait_Name，由此可知，
// Trait其实是泛型的数据类型，Trait限制了泛型所能代表的类型，正如数据类型限制了变量所能存放的数据

// 只限制泛型T是实现了std::ops::Add Trait的类型还不够，还要限制它实现了Copy Trait
// 以便函数体内的参数i被转移所有权时会自动进行Copy，但Copy Trait是Clone Trait的子Trait，
// 即Copy依赖于Clone，因此限制泛型T实现Copy的同时，还要限制泛型T同时实现Clone Trait

// 如果参数是一个引用，且又使用泛型，则需要使用泛型的引用&T或&mut T

use std::fmt::Display;

fn f<T: Display>(i: &T) {
    println!("{}", *i);
}

fn main() {
    // rustc在编译代码时，会将所有的泛型替换成它所代表的具体数据类型，
    // 就像编译期间会将变量名替换成它所代表数据的内存地址一样

    // 那么编译期间，编译器会对应生成2个double_me()函数，它们的参数类型分别是 i16, i32
    println!("{}", double(3_i16));
    println!("{}", double(3_i32));

    // 由于编译期间已经将泛型替换成了具体的数据类型，因此，在程序运行期间，
    // 直接调用对应类型的函数即可，不需要再消耗任何额外的资源去计算泛型所代表的具体类型。
    // 因此，Rust的泛型是零运行时开销的
}
