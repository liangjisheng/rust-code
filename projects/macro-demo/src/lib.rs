use proc_macro::TokenStream;
// 通过语法树(AST)生成rust代码
use quote::quote;
// 将代码解析成语法树(AST)
use syn::{parse_macro_input, DeriveInput, Ident};

// 注意 宏定义所在的 crate ,不能和你使用它所在的 crate 是同一个 crate

// 要理解TokenStream，需要一些简单的编译原理知识。编译器在编译一段程序时，会首先将输入的文本转换成
// 一系列的Token（标识符、关键字、符号、字面量等），同时忽略注释（文档注释除外）与空白字符等
// TokenStream顾名思义，是Rust中对一系列连续的Token的抽象。在宏展开的过程中，
// 遇到派生宏时，会将整个结构体（或enum、union）展开成TokenStream作为派生宏函数的输入，
// 然后将其输出的TokenStream附加到结构体后面，再继续作语法分析

// Rust 中的自定义派生宏允许自动实现 traits。这些宏使你能够使用 #[derive(Trait)] 实现特征
// 派生宏生成代码，然后将其添加到同一模块中，而属性宏生成代码来替换它们所应用的项目。派生宏用于扩展，属性宏用于替换

// 属性宏
// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    // return a simple TokenStream for Struct
    TokenStream::from(quote! {struct H{}})
}

// 属性宏 (两个参数)
#[proc_macro_attribute]
pub fn log_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Attr:{}", attr.to_string());
    println!("Item:{}", item.to_string());
    item
}

// 函数式宏类似于声明式宏，因为他们都通过宏调用操作符!来执行，并且看起来都像是函数调用。它们都作用于圆括号里的代码
// 输入的TokenStream是宏调用的分隔符内的内容，输出的TokenStream替换整个宏调用
#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
        fn anwser() -> i32 {
            5
        }
    ))
}

#[proc_macro]
/// 该宏的作用是生成一个函数，函数名是hello
pub fn say_hello(item: TokenStream) -> TokenStream {
    "fn hello() -> &'static str { \"hello\" }".parse().unwrap()
}

#[proc_macro]
pub fn make_hello(item: TokenStream) -> TokenStream {
    let name = item.to_string();
    let hello = "Hello ".to_string() + name.as_ref();
    let fn_name =
        "fn hello_".to_string() + name.as_ref() + "(){ println!(\"" + hello.as_ref() + "\"); }";
    fn_name.parse().unwrap()
}

// 派生宏
#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    println!("hello_derive: {:?}", input);
    TokenStream::new()
    // 如果直接返回input，编译会报重复定义，说明派生宏用于扩展定义
    // input
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Trait)]
pub fn derive_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Trait for #name {
            fn print(&self) -> usize {
                println!("hello from {}", stringify!(#name));
                5
           }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// 派生宏
#[proc_macro_derive(Builder)]
pub fn derive1(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());
    quote! (
        impl #ident {
            pub fn builder() -> #ident_builder {
                #ident_builder
            }
        }

        pub struct #ident_builder;
    )
    .into()
}

// #[proc_macro_derive(Builder)]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let _ = input;
//
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;
}
