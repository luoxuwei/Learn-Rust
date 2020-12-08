
## Rust通用编程概念

- [Rust变量](./doc/base.md)

- [Rust类型](./doc/base.md)

- [Rust注释](./doc/base.md)
- [Rust控制流](./doc/base.md)

- [Rust函数](./doc/base.md)

## 所有权、引用、slice
- [所有权](./doc/own.md)
- [引用](./doc/ref.md)
- [slice](./doc/slice.md)

## 结构体
- [Rust结构体](./doc/stru.md)
- [Rust方法](./doc/stru_method.md)

## 枚举与模式匹配
- [枚举与模式匹配](./doc/enum.md)
- [Option](./doc/option.md)

## vector、String、HashMap
- [vector](./doc/vector.md)
- [String](./doc/string.md)
- [HashMap](./doc/hashmap.md)

## 包、模块
- [包/模块](./doc/crate.md)
- [例子](./demo/crate)

## 错误处理
- [错误处理](./doc/error.md)
- [传播错误](./doc/error.md)

## 测试
- [测试](./doc/test.md)

## 泛型
- [函数中使用泛型](./doc/generics.md)
- [结构体和方法中使用泛型](./doc/generics.md)

## 特征
- [trait 默认实现](./doc/trait.md)
- [trait bound](./doc/trait.md)
- [trait 作为返回值](./doc/trait.md)
- [有条件的实现方法 泛型约束](./doc/trait.md)
- [有条件的实现 trait 类比 类的继承](./doc/trait.md)

## 生命周期
- [方法中的生命周期](./learn_life1/src/main.rs)
- [生命周期标注省略](./learn_life2/src/main.rs)
- [结构体中的生命周期](./learn_life3/src/main.rs)
- [函数中的生命周期](./learn_life4/src/main.rs)
- [为带有生命周期标注的struct实现trait的问题](https://www.bilibili.com/read/cv4263006)

## 闭包
- [闭包](./learn_closure1/src/main.rs)
- [闭包的例子](./learn_closure2/src/main.rs)
- [闭包获取环境中的变量](./learn_closure3/src/main.rs)

## 迭代器
- [迭代器](./learn_iter1/src/main.rs)
- [自定义迭代器](./learn_iter2/src/main.rs)

## cargo 自定义构建
- [cargo自定义构建](./learn_cargo1)

## 包的使用和发布
- [使用别人的包](https://www.bilibili.com/read/cv4431403)
- [文档注释](https://www.bilibili.com/read/cv4441798)
- [crate的发布与撤回](https://www.bilibili.com/read/cv4462484)

## Cargo 工作空间
- [Cargo 工作空间](./rust-workspace)
  cargo run -p addr

## 智能指针
- [智能指针](./rust-pointer)

## 文件读写

- [文件读写](https://www.bilibili.com/read/cv4256536)

## BOX

- [box介绍](./learn_box/src/main.rs)
- [使用box](./learn_box1/src/main.rs)
- [解引用](./learn_deref/src/main.rs)
- [解引用多态](./learn_deref1/src/main.rs)

## drop trait 

- [drop trait 清理代码](./learn_drop/src/main.rs)
- [drop提前释放](./learn_drop1/src/main.rs)

## Rc智能指针

- [Rc智能指针使用](./learn_rc1/src/main.rs)
- [Rc智能指针深入](./learn_rc2/src/main.rs)

## RefCell

- [RefCell介绍](./learn_refcell/src/main.rs)

## 引用循环

- [引用循环](./learn_loop_ref/src/main.rs)

- [弱引用](./learn_loop_ref2/src/main.rs)

- [树形结构](./learn_weak1/src/main.rs)
- [树形结构](./learn_weak2/src/main.rs)

## 多线程

- [线程与move闭包](./learn_thread2/src/main.rs)
- [等待线程结束](./learn_thread/src/main.rs)

## 通道

- [通道介绍](./learn_channel/src/main.rs)

- [send move](./learn_channel2/src/main.rs)

- [发送多个值](./learn_channel3/src/main.rs)

- [多个生成者单个消费者](./learn_channel4/src/main.rs)

## 互斥器

- [互斥器介绍](./learn_thread3/src/main.rs)
- [多线程间使用互斥器](./learn_thread4/src/main.rs)



## send和sync trait 介绍

- [send 和 sync trait 介绍](https://www.bilibili.com/read/cv4754537)

## 面向对象

- [对象](./learn_oo/src/main.rs)

- [封装](./learn_oo1)



## trait 对象

- [trait 对象](./learn_oo2)

- [非对象安全](./learn_oo3)



## 模式

- [模式](./learn_pattern)

## 模式的可反驳和不可反驳

- [模式的可反驳和不可反驳](./learn_pattern1)

##  所有模式语法

- [字面/命名变量/多个模式](./learn_pattern2)
- [...= 匹配](./learn_pattern2)

- [解构并分解结构体值](./learn_pattern3)

- [解构枚举类型](./learn_pattern4)

- [解构嵌套枚举类型](./learn_pattern5)

- [解构嵌套的结构体和元组](./learn_pattern6)

- [用下划线忽略模式中的值](./learn_pattern7)

- [用点点忽略模式中的值](./learn_pattern8)

- [匹配守卫](./learn_pattern9)

- [@运算符](./learn_pattern10)

## 不安全代码介绍

- [不安全代码介绍](https://www.bilibili.com/read/cv4915116)

- [解引用裸指针](./learn_unsafe1)
- [不安全函数调用方法](./learn_unsafe2)
- [不安全函数及FFI rust 调用 C 函数](./learn_unsafe3)

- [c 函数调用rust](./learn_unsafe4)  https://www.bilibili.com/read/cv4948383
- [访问修改静态变量](./learn_unsafe5)

- [实现不安全的trait](./learn_unsafe6)

## 关联类型

- [在trait定义中指定站位符](./learn_htrait1)

- [默认泛型参数与运算符重载](./learn_htrait2)
- [完全限定语法](./learn_htrait3)

- [父trait](./learn_htrait4)
- [newtype模式](./learn_htrait5)

## 类型

- [类型别名](./learn_htype1)

- [never type 介绍](./learn_htype2)
- [动态大小类型和Sized trait](./learn_htype3)

## 高级函数和闭包

- [函数指针](./learn_hfn1)

- [返回闭包](./learn_hfn2)



## 宏

- [宏介绍](https://www.bilibili.com/read/cv5129293)

- [声明宏 ](./learn_macro1) https://www.bilibili.com/read/cv5151988

- [自定义derive宏](./learn_macro2)  https://www.bilibili.com/read/cv5160408

- 类属性宏和类函数宏

## 实战

[rust-algorithms](https://github.com/EbTech/rust-algorithms)

[TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)
