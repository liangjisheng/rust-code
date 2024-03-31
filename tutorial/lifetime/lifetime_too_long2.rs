// https://course.rs/compiler/fight-with-compiler/lifetime/too-long2.html
// 继上篇文章后，我们再来看一段可能涉及生命周期过大导致的无法编译问题:

fn bar(writer: &mut Writer) {
    baz(&mut writer.indent());
    writer.write("world");
}

fn baz(writer: &mut Writer) {
    writer.write("hello");
}

pub struct Writer<'a> {
    target: &'a mut String,
    indent: usize,
}

impl<'a> Writer<'a> {
    // fn indent(&'a mut self) -> &'a mut Self {
    //     &mut Self {
    //         target: self.target,
    //         indent: self.indent + 1,
    //     }
    // }

    // fn indent(&'a mut self) -> Writer<'a> {
    //     Writer {
    //         target: self.target,
    //         indent: self.indent + 1,
    //     }
    // }

    // 这里稍微展开以下，为何indent方法返回值的生命周期不能与参数中的self相同。
    // 首先，我们假设它们可以相同，也就是上面的代码可以编译通过，由于此时在返回值
    // 中借用了self的可变引用，意味着如果你在返回值被使用后，还继续使用self 会
    // 导致重复借用的错误，因为返回值的生命周期将持续到 self 结束。

    //声明返回值的生命周期为另一个，与 'a 不同
    fn indent<'b>(&'b mut self) -> Writer<'b> {
        Writer {
            target: self.target,
            indent: self.indent + 1,
        }
    }

    //回想下生命周期消除的规则，我们还可以实现的更优雅
    // fn indent(&mut self) -> Writer {
    //     Writer {
    //         target: self.target,
    //         indent: self.indent + 1,
    //     }
    // }

    fn write(&mut self, s: &str) {
        for _ in 0..self.indent {
            self.target.push(' ');
        }
        self.target.push_str(s);
        self.target.push('\n');
    }
}

fn main() {}
