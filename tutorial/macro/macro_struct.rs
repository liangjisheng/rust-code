// 在 Rust 中使用声明式宏进行高级解析

// $vis将会拥有可见性，$struct_name将会拥有一个结构体名
// 为了让一个结构体是公开的，我们只需要添加pub关键字并忽略$vis变量

macro_rules! make_public {
    (
  // use vis type for visibility keyword and ident for struct name
     $vis:vis struct $struct_name:ident { }
    ) => {{
        pub struct $struct_name {}
    }};
}

// 一个struct可能包含多个字段，这些字段具有相同或不同的数据类型和可见性。
// ty token 类型用于数据类型，vis用于可见性，ident用于字段名。我们将会使用*用于零个或更多字段
macro_rules! make_public_1{
    (
     $vis:vis struct $struct_name:ident {
        $(
 // vis for field visibility, ident for field name and ty for field data type
        $field_vis:vis $field_name:ident : $field_type:ty
        ),*
    }
    ) => {
        {
            pub struct $struct_name{
                $(
                pub $field_name : $field_type,
                )*
            }
        }
    }
}

// 通常，struct有一些附加的元数据或者过程宏，比如#[derive(Debug)]。这个元数据需要保持完整
// 解析这类元数据是通过使用meta类型来完成的
macro_rules! make_public_2{
    (
     $(#[$meta:meta])*
     $vis:vis struct $struct_name:ident {
        $(
        $(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+
    }
    ) => {

            $(#[$meta])*
            pub struct $struct_name{
                $(
                $(#[$field_meta:meta])*
                pub $field_name : $field_type,
                )*
            }
    }
}

fn main() {
    make_public_2! {
        #[derive(Debug)]
        struct Name{
            n:i64,
            t:i64,
            g:i64,
        }
    }
}
