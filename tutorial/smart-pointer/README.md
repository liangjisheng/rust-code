# smart pointer

[smart pointers](https://kaisery.github.io/trpl-zh-cn/ch15-00-smart-pointers.html)
[Box](https://blog.csdn.net/Konquerx/article/details/115480957)
[Box::leak](https://www.zhihu.com/question/511520023)
[cell](https://zhuanlan.zhihu.com/p/598708941)

Box <T>是一个智能指针，指向在类型为T的堆上分配的数据，其中 T 是数据的类型。它用于将数据存储在堆上而不是堆栈上  
Deref <T>：Deref <T>是一个智能指针，用于自定义解除引用运算符(*)的行为  
Drop <T>：Drop <T>是一个智能指针，用于在变量超出范围时从堆内存中释放空间  
Rc <T>：Rc <T>代表参考计数指针。它是一个智能指针，用于记录存储在堆上的值的引用数  
RefCell <T>: RefCell <T>是一个智能指针，允许借用可变数据，即使数据是不可变的。这个过程被称为内部可变性  
