use std::fmt::Debug;

trait Playable: Debug {
    fn play(&self);
    fn pause(&self) {
        println!("pause");
    }
    fn get_duration(&self) -> f32;
}

// 要让Playable实现名为std::fmt::Debug的Trait，因为Playable自身也是一个Trait
// 所以使用Trait继承的方式来继承Debug。继承Debug后，要求实现Playable Trait
// 的类型也都要实现Debug Trait，因此在Audio和Video之前使用#[derive(Debug)]来实现Debug Trait

// Audio类型，实现Trait Playable
#[derive(Debug)]
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

// Video类型，实现Trait Playable
#[derive(Debug)]
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
    let a: &dyn Playable = &Audio {
        name: "telephone.mp3".to_string(),
        duration: 3.42,
    };

    let b: &dyn Playable = &Video {
        name: "alice.mp4".to_string(),
        duration: 59.59,
    };

    let arr: [&dyn Playable; 2] = [a, b];
    println!("{:#?}", arr);
}
