// 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型
// 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的
// 元组结构体定义语法
// struct TupleName(type1, type2, ..., typen);
// 实例化元组结构体语法
// let var = TupleName(value1, value2, ... valuen);

// 不同名字元组结构体之间哪怕每个字段下的属性完全相同，也是属于不同类型的元组结构体，它们之间的实例不能互通。
// 元组结构体的使用与普通的元组大致相同，比如可以用模式匹配将其解构为单独的部分，也可以使用.下标的形式访问单独的值等

// 类单元结构体没有任何的字段，与unit类型()类似。
// 类单元结构体常常在开发者想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
// 即实现一种类型，但是这种类型没有任何字段，只有行为
