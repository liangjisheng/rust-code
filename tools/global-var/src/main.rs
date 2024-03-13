use rand;
use std::collections::HashMap;
use std::io::{self, Write};
use std::ptr::hash;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rand::Rng;

// Rust 对全局变量的使用有许多限制。包括
// 全局变量必须在声明的时候马上初始化
// 全局变量的初始化必须是简单赋值，不能包括复杂的表达式、语句和函数调用
// 带有 mut 修饰的全局变量，在使用的时候必须使用 unsafe

//全局变量的内存不是分配在当前函数栈上，函数退出的时候，并不会销毁全局变量占用的内存空间，程序退出才会回收

// Rust 不允许用户在 main 函数之前或者之后执行自己的代码。所以，
// 比较复杂的 static 变量的初始化，一般需要使用 lazy 方式，在第一次使用的时候初始化

// 使用 const 声明的是常量，而不是变量。它与 static 变量的最大区别在于，
// 编译器并不一定会给 const 常量分配内存空间，在编译过程中，它很可能会被内联优化
// 所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
const GLOBAL_1: i32 = 1;

static GLOBAL: i32 = 0;
//可变全局变量无论读写都必须用 unsafe 修饰。
static mut G2: i32 = 4;

static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

// 运行期初始化
// 通过lazy_static宏定义的变量可以使用任何表达式来初始化
// 使用这个宏,所有 static类型的变量可在执行的代码在运行时被初始化
// 这包括任何需要堆分配,如vector或hash map,以及任何非常量函数调用

// 使用lazy_static在每次访问静态变量时，会有轻微的性能损失，因为其内部实现用了一个底层的并发原语
// std::sync::Once，在每次访问该变量时，程序都会执行一次原子指令用于确认静态变量的初始化是否完成
// lazy_static宏，匹配的是static ref，所以定义的静态变量都是不可变引用

lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Alice, Jack, Allen"));
}

fn times_two(n: u32) -> u32 {
    n * 2
}

fn main() {
    // f1();
    // f2();
    // f3();
    // f4();
    f5();
}

fn f1() {
    println!("GLOBAL: {}", GLOBAL);
    println!("GLOBAL_1: {}", GLOBAL_1);

    for _ in 0..100 {
        PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);
    }
    println!("Packets served = {:?}", PACKETS_SERVED);

    for i in 0..10 {
        HOSTNAME.lock().unwrap().push_str(&format!("{}", i));
    }
    println!("HOSTNAME = {:?}", HOSTNAME.lock());

    unsafe {
        println!("G2: {}", G2);
        G2 = 5;
        println!("G2: {}", G2);
    }

    // 线程局部变量
    // 每次调用 thread_rng 函数的时候，都是返回的是指向同一个生成器的指针，避免重复构造
    let mut rng = rand::thread_rng();
    println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(b"hello world\n").expect("write error");

    println!("The map has {} entries.", *COUNT);
    // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!(
        "An expensive calculation on a static results in: {}.",
        *NUMBER
    );
    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("{:?}", *HASHMAP);
}

//静态常量
const MAX_ID: usize = usize::MAX / 2;

// 用一个变量来统计程序当前的总请求数
//静态变量
static mut REQUEST_RECV: usize = 0;

// 静态变量和常量的区别
// 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
// 存储在静态变量中的值必须要实现 Sync trait

// 想要全局计数器、状态控制等功能，又想要线程安全的实现，原子类型是非常好的办法
static REQUEST_RECV_ATOM: AtomicUsize = AtomicUsize::new(0);

fn f2() {
    println!("用户ID允许的最大值是 {}", MAX_ID);

    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }

    for _ in 0..100 {
        REQUEST_RECV_ATOM.fetch_add(1, Ordering::Relaxed);
    }
    println!("当前用户请求数 {:?}", REQUEST_RECV_ATOM);
}

struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_id() -> usize {
    // 检查两次溢出，否则直接加一可能导致溢出
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

fn f3() {
    let factory = Factory::new();
    println!("{}", factory.factory_id);
    println!("{}", generate_id());
}

fn f4() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}

// Rust为我们提供了Box::leak方法，它可以将一个变量从内存中泄漏(听上去怪怪的，竟然做主动内存泄漏)
// 然后将其变为 'static 生命周期，最终该变量将和程序活得一样久，因此可以赋值给全局静态变量CONFIG

#[derive(Debug, Clone)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;
static mut CONFIG1: Option<&mut Config> = None;

// 从函数中返回全局变量
fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    // 将`c`从内存中泄漏，变成`'static`生命周期
    Some(Box::leak(c))
}

fn f5() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);

        CONFIG1 = init();
        println!("{:?}", CONFIG1)
    }
}
