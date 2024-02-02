# lifetime

生命周期，简而言之就是引用的有效作用域。在大多数时候，我们无需手动的声明生命周期，因为编译器可以自动进行推导

[lifetime](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)
[lifetime](https://zhuanlan.zhihu.com/p/93193353)
[lifetime](https://zhuanlan.zhihu.com/p/93846179)
[lifetime](https://zhuanlan.zhihu.com/p/104742696)
[lifetime](https://blog.csdn.net/qq_46878411/article/details/123045316)
[lifetime](https://blog.csdn.net/CAir2/article/details/127443657)
[static](https://www.zhihu.com/question/515953625/answer/2343571646)

Rust有个Borrow检查器用于检查Borrow对象（或者叫引用）是否有效  
引用的生命周期不能大于所引用值的生命周期
Rust 编译器有一个 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的
