// Trait最基本的作用是从多种类型中抽取出共性的属性或方法，并定义这些方法的规范(即方法签名)

// 数值类型(包括i32、u32、f32等等)的加减乘除功能，也都是通过实现各种对应的Trait而来的
// 比如，为了支持加法操作+，这些数值类型都实现了std::ops::Add这个Trait。可以这样理解
// std::ops::Add Trait是一种通用功能，只要某个类型(包括自定义类型)实现了std::ops::Add这个Trait
// 这个类型的实例对象就可以使用加法操作。同理，对减法、除法、乘法、取模等等操作，也都如此

use std::io::Write;

trait Playable {
    fn play(&self);
    fn pause(&self) {
        println!("pause");
    }
    fn get_duration(&self) -> f32;
}

struct Audio {
    name: String,
    duration: f32,
}

impl Playable for Audio {
    fn play(&self) {
        println!("listening audio: {}", self.name);
    }
    fn get_duration(&self) -> f32 {
        self.duration
    }
}

struct Video {
    name: String,
    duration: f32,
}

impl Playable for Video {
    fn play(&self) {
        println!("watching video: {}", self.name);
    }
    fn pause(&self) {
        println!("video paused");
    }
    fn get_duration(&self) -> f32 {
        self.duration
    }
}

fn main() {
    let audio = Audio {
        name: "telephone.mp3".to_string(),
        duration: 4.32,
    };
    audio.play();
    // 而对于Audio没有定义的pause方法，则会从其所实现的Trait中寻找
    audio.pause();
    println!("{}", audio.get_duration());

    let video = Video {
        name: "alice.mp4".to_string(),
        duration: 59.59,
    };
    video.play();
    video.pause();
    println!("{}", video.get_duration());
}

// 但对于一些常见的Trait，可在Struct类型或Enum类型前使用#[derive()]简单方便地实现这些Trait
// Rust会自动为Struct类型和Enum类型定义好这些Trait所要求实现的方法

#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 现在，Person类型和Direction类型就都实现了Copy Trait和Clone Trait，
// 具备了这两个Trait的功能：所有权转移时是可拷贝的、可克隆的

// Rust允许在任何时候为任何类型实现任何Trait。例如，在自己的代码中为标准库Vec类型实现trait A
// 这使得编程人员可以非常方便地为某类型添加功能，无论这个功能来自自定义的Trait还是Rust中已存在的Trait，
// 也无论这个类型是自定义类型还是Rust内置类型

// 但对于Rust而言，当类型A实现了Trait T时，想要通过A的实例对象来调用来自于T的方法时，
// 要求Trait T必须在当前作用域内，否则报错

fn t1() {
    // Vec类型已经实现了std::io::Write
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello"); // 报错：未找到write_all方法

    // 上面的代码报错是因为Vec虽然实现了Trait Write，但Write并未在作用域内，
    // 因此调用来自Write的方法write_all会查找不到该方法
    // 加上use std::io::Write即可

    // 为什么Rust要做如此要求呢？这可以避免冲突。比如张三可以在他的代码中为u8类型实现Trait A
    // 并定义了实现A所需的方法f，张三导入使用的第三方包中可能也为u8类型实现了Trait A
    // 毕竟Rust允许在任何位置为某类型实现某Trait。因此，张三执行(3_u8).f()的时候
    // Rust必须要能够区分调用的这个f方法来自于何处
}
