// 自引用结构有一个很大的问题是：一旦它被移动，原本的指针就会指向旧的地址。
// 所以需要有某种机制来保证这种情况不会发生，Pin 就是为这个目的而设计的一
// 个数据结构，可以 Pin 住指向一个 Future 的指针

// 自引用数据结构并非只在异步代码里出现，只不过异步代码在内部生成用状态机表述的
// Future 时，很容易产生自引用结构。看一个和 Future 无关的自引用数据结构的例子

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后指向 name
    name_ptr: *const String,
}

// 但一旦对这个结构做 move 操作，name_ref 指向的位置依然是
// move 前 name 的地址，这就引发了问题

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String;
    }

    pub fn print_name(&self) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            // 在使用 ptr 是需要 unsafe
            // SAFETY: 这里 name_ptr 潜在不安全，会指向旧的位置
            unsafe { &*self.name_ptr },
        );
    }
}

fn main() {
    let data = move_creates_issue();
    println!("data: {:?}", data);
    // 如果把下面这句注释掉，程序运行会直接 segment error
    // data.print_name();
    println!();

    mem_swap_creates_issue();
}

fn move_creates_issue() -> SelfReference {
    let mut data = SelfReference::new("alice");
    data.init();

    // 不 move，一切正常
    data.print_name();

    let data = move_it(data);

    // move 之后，name_ref 指向的位置是已经失效的地址
    // 只不过现在 move 前的地址还没被回收挪作它用
    data.print_name();
    data
}

// 同样的，如果我们使用 std::mem:swap，也会出现类似的问题，一旦 swap
// 两个数据的内容交换，然而，由于 name_ref 指向的地址还是旧的，所以整个指针体系都混乱了

fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("alice");
    data1.init();

    let mut data2 = SelfReference::new("bob");
    data2.init();

    data1.print_name();
    data2.print_name();

    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn move_it(data: SelfReference) -> SelfReference {
    data
}
