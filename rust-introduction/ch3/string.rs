// Rust有两种字符串类型：str和String。其中str是String的切片类型，
// 也就是说，str类型的字符串值是String类型的字符串值的一部分或全部
// str字符串是String类型字符串的切片(slice)类型

fn main() {
    // 字符串字面量使用双引号包围
    let s = "alice";
    // 等价于
    // let s: &str = "alice";
    println!("{}", s);

    // 实际上，字符串字面量的数据类型均为&str，其中str表示str类型，&表示该类型的引用，
    // 即一个指针。因此，&str表示的是一个指向内存中str类型数据的指针，
    // 该指针所指向的内存位置处保存了字符串数据

    // 类型自动推导为: String
    let s = String::from("alice");
    let s1 = "alice".to_string();
    println!("{},{}", s, s1);

    let mut s = String::from("alice");
    s.push('.'); // push()可追加单个char字符类型
    s.push_str("com"); // push_str()可追加&str类型的字符串
    println!("{}", s); // 输出：alice.com

    let s = String::from("alice");
    // 自动推导数据类型为&str
    //   s[0..3]的类型为str
    //  &s[0..3]的类型为&str
    // 等价于&(s[0..3])而不是(&s)[0..3]
    let s_str = &s[0..3];
    // 现在s_str通过胖指针引用了源String字符串中的局部数据
    println!("{}", s_str); // 输出: ali

    // 字符串字面量的类型是&str类型。也就是说，字符串字面量实际上是字符串切片类型的引用类型
    // 编译器编译的时候直接将字符串字面量以硬编码的方式写入程序二进制文件中，当程序被加载时，
    // 字符串字面量被放在内存的某个位置(不在堆中也不在栈中，而是在类似于静态数据区的全局字面量区)。
    // 当程序执行到let s="hello";准备将其赋值给变量s时(注：s在栈上)，直接将字面量内存区的
    // 该数据地址保存到&str类型的s中

    // 当程序运行到let s = String::from()操作时，从字面量内存区将其拷贝到堆内存中，
    // 然后将堆内存中该数据的地址保存到栈内变量s中
}
