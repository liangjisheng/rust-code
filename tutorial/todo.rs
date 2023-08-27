trait Foo {
    fn bar(&self);
    fn baz(&self);
}

struct MyStruct;

impl Foo for MyStruct {
    fn bar(&self) {
        // implementation goes here
        println!("bar");
    }

    fn baz(&self) {
        // let's not worry about implementing baz() for now
        todo!();
    }
}

fn main() {
    let s = MyStruct;
    s.bar();

    // we aren't even using baz(), so this is fine.
    // call s.baz will panic
}
