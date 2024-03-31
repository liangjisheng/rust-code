// https://course.rs/profiling/performance/deep-into-move.html
// move时发生了数据的深拷贝

mod m1 {
    struct LargeArray {
        a: [i128; 10000],
    }

    impl LargeArray {
        #[inline(always)]
        fn transfer(mut self) -> Self {
            println!("{:?}", &mut self.a[1] as *mut i128);

            // 改变数组中的值
            self.a[1] += 23;
            self.a[4] += 24;

            // 返回所有权
            self
        }
    }

    pub fn demo1() {
        let mut f = LargeArray { a: [10i128; 10000] };

        println!("{:?}", &mut f.a[1] as *mut i128);

        let mut f2 = f.transfer();

        println!("{:?}", &mut f2.a[1] as *mut i128);
    }
}

mod m2 {
    struct LargeArray {
        a: Box<[i128; 10000]>,
    }

    impl LargeArray {
        #[inline(always)]
        fn transfer(mut self) -> Self {
            println!("{:?}", &mut self.a[1] as *mut i128);

            //do some stuff to alter it
            self.a[1] += 23;
            self.a[4] += 24;

            //return the same object
            self
        }
    }

    pub fn demo2() {
        let mut f = LargeArray {
            a: Box::new([10i128; 10000]),
        };

        println!("{:?}", &mut f.a[1] as *mut i128);

        let mut f2 = f.transfer();

        println!("{:?}", &mut f2.a[1] as *mut i128);
    }
}

fn main() {
    // m1::demo1();
    // LargeArray没有实现Copy特征，因此在所有权转移时， 本应该只是复制一下引用，
    // 底层的数组并不会被复制
    // 0x7ffee45897d8
    // 0x7ffee45feb20
    // 0x7ffee45d7a20
    // 但输出却是3个不同的地址，结构体是一个复合类型，它内部字段的数据存在哪里，
    // 就大致决定了它存在哪里。而该结构体里面的a字段是一个数组，而不是动态数组Vec，
    // 从数组章节可知：数组是存储在栈上的数据结构！
    // 再有栈上的数据在move的时候，是要从一个栈复制到另外一个栈的，那内存地址就变了

    // 解决方法之一是使用&mut self进行可变借用，而不是转移进来所有权，再转移出去
    // 另一个就是使用 Box 分配到堆上
    m2::demo2();
    // 0x7fbae8008010
    // 0x7fbae8008010
    // 0x7fbae8008010
}
