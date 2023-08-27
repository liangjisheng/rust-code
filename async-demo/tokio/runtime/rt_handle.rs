// tokio提供了一个称为runtime Handle的东西，它实际上是runtime的一个引用，可以随意被clone
// 它可以spawn()生成异步任务，这些异步任务将绑定在其所引用的runtime中，还可以 block_on() 或
// enter()进入其所引用的 runtime, 此外，还可以生成 blocking thread

use tokio::runtime;

fn main() {
    let rt = runtime::Runtime::new().unwrap();
    let handle = rt.handle();
    handle.spawn(async {});
    handle.spawn_blocking(|| {});
    handle.block_on(async {});
    let _guard = handle.enter();
}

// 需注意，如果runtime已被关闭，handle也将失效，此后再使用handle，将panic
