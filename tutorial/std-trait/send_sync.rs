// 标记 Trait（Marker Trait）
// 自动 Trait （Auto Trait）
// 不安全 Trait （Unsafe Trait）
// unsafe auto trait Send {}
// unsafe auto trait Sync {}

// 如果一个类型是Send，这就意味着它可以在线程之间被安全地发送（send）。如果一个类型是Sync，
// 这就意味着它可以在线程间安全地共享引用。说得更准确点就是，当且仅当&T是Send时，类型T是Sync

// 我们可以传递同一份数据的若干个不可变引用到多个线程中，由于只要有不可变引用存在，
// Rust 就会静态地保证底层数据不被修改，所以我们可以保证不会发生数据竞争

mod m1 {
    use crossbeam::thread;

    pub fn t1() {
        let mut greeting = String::from("Hello");
        let greeting_ref = &greeting;

        thread::scope(|scoped_thread| {
            // spawn 3 threads
            for n in 1..=3 {
                // greeting_ref copied into every thread
                scoped_thread.spawn(move |_| {
                    println!("{} {}", greeting_ref, n); // prints "Hello {n}"
                });
            }

            // line below could cause UB or data races but compiler rejects it
            // greeting += " world"; // ❌ cannot mutate greeting while immutable refs exist
        });

        // can mutate greeting after every thread has joined
        greeting += " world"; // ✅
        println!("{}", greeting); // prints "Hello world"
    }
}

// 同样地，我们可以把数据的一个可变引用传递给一个单独的线程，由于 Rust 静态地保证不存在可变引用的别名，
// 所以底层数据不会通过另一个可变引用被修改，因此我们也可以保证不会发生数据竞争
mod m2 {
    use crossbeam::thread;

    pub fn t2() {
        let mut greeting = String::from("Hello");
        let greeting_ref = &mut greeting;

        thread::scope(|scoped_thread| {
            // greeting_ref moved into thread
            scoped_thread.spawn(move |_| {
                *greeting_ref += " world";
                println!("{}", greeting_ref); // prints "Hello world"
            });

            // line below could cause UB or data races but compiler rejects it
            // greeting += "!!!"; // ❌ cannot mutate greeting while mutable refs exist
        });

        // can mutate greeting after the thread has joined
        greeting += "!!!"; // ✅
        println!("{}", greeting); // prints "Hello world!!!"
    }
}

fn main() {
    m1::t1();
}
