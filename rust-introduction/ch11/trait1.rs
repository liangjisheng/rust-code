// 了解Trait Object之后，使用它就不再难了，它也只是一种数据类型罢了

trait Playable {
    fn play(&self);
    fn pause(&self) {
        println!("pause");
    }
    fn get_duration(&self) -> f32;
}

// Audio类型，实现Trait Playable
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

// 将Audio的实例或Video的实例当作Playable的Trait Object来使用
fn main() {
    let x: &dyn Playable = &Audio {
        name: "telephone.mp3".to_string(),
        duration: 3.42,
    };
    x.play();
    // x的数据类型是Playable的Trait Object类型的引用，它在栈中保存了一个指向Audio实例数据的指针
    // 还保存了一个指向包含了它可调用方法的vtable的指针

    let y: &dyn Playable = &Video {
        name: "alice.mp4".to_string(),
        duration: 59.59,
    };
    y.play();
}
