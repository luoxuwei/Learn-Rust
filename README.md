
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
- [方法中的生命周期](./doc/lifetime.md)
- [生命周期标注省略](./doc/lifetime.md)
- [结构体中的生命周期](./doc/lifetime.md)
- [函数中的生命周期](./doc/lifetime.md)
- [为带有生命周期标注的struct实现trait的问题](./doc/lifetime.md)

## 闭包
- [闭包](./doc/closure.md)
- [闭包的例子](./doc/closure.md)
- [闭包获取环境中的变量](./doc/closure.md)

## 迭代器
- [迭代器](./doc/iter.md)
- [自定义迭代器](./doc/iter.md)

## cargo 自定义构建
- [cargo自定义构建](./doc/cargo.md)

## 包的使用和发布
- [使用别人的包](./doc/cargo_comment_crate.md)
- [文档注释](./doc/cargo_comment_crate.md)
- [crate的发布与撤回](./doc/cargo_comment_crate.md)

## Cargo 工作空间
- [Cargo 工作空间](./doc/workspace.md)
  cargo run -p addr
- [例子](./demo/addr)

## 智能指针
- [智能指针](./doc/pointer.md)

## 文件读写

- [文件读写](./doc/file_io.md)

## BOX

- [box介绍](./doc/box.md)
- [使用box](./doc/box.md)
- [解引用](./doc/deref.md)
- [解引用多态](./doc/deref.md)

## drop trait 

- [drop trait 清理代码](./doc/drop_trait.md)
- [drop提前释放](./doc/drop_trait.md)

## Rc智能指针

- [Rc智能指针使用](./doc/rc.md)
- [Rc智能指针深入](./doc/rc.md)

## RefCell

- [RefCell介绍](./doc/refcell.md)

## 引用循环

- [引用循环](./doc/loop_ref.md)

- [弱引用](./doc/weak_ref.md)

- [树形结构](./doc/weak_ref.md)

## 多线程

- [线程与move闭包](./doc/thread.md)
- [等待线程结束](./doc/thread.md)

## 通道

- [通道介绍](./doc/channel.md)

- [send move](./doc/channel.md)

- [发送多个值](./doc/channel.md)

- [多个生成者单个消费者](./doc/channel.md)

## 互斥器

- [互斥器介绍](./doc/mutex.md)
- [多线程间使用互斥器](./doc/mutex.md)

## send和sync trait 介绍

- [send 和 sync trait 介绍](./doc/sync_send_trait.md)

## 面向对象

- [对象](./doc/oo.md)

- [封装](./doc/oo.md)

## trait 对象

- [trait 对象](./doc/oo_trait.md)

- [非对象安全](./doc/oo_trait.md)

## 模式

- [模式](./doc/pattern.md)

## 模式的可反驳和不可反驳

- [模式的可反驳和不可反驳](./doc/refutability_refutability.md)

##  所有模式语法

- [字面/命名变量/多个模式](./doc/pattern_all.md)
- [...= 匹配](./doc/pattern_all.md)

- [解构并分解结构体值](./doc/pattern_all.md)

- [解构枚举类型](./doc/pattern_all.md)

- [解构嵌套枚举类型](./doc/pattern_all.md)

- [解构嵌套的结构体和元组](./doc/pattern_all.md)

- [用下划线忽略模式中的值](./doc/pattern_all.md)

- [用点点忽略模式中的值](./doc/pattern_all.md)

- [匹配守卫](./doc/pattern_all.md)

- [@运算符](./doc/pattern_all.md)

## 不安全代码介绍

- [不安全代码介绍](./doc/unsafe.md)

- [解引用裸指针](./doc/unsafe.md)
- [不安全函数调用方法](./doc/unsafe.md)
- [不安全函数及FFI rust 调用 C 函数](./doc/unsafe.md)

- [c 函数调用rust](./doc/unsafe.md)
- [访问修改静态变量](./doc/unsafe.md)

- [实现不安全的trait](./doc/unsafe.md)

## 关联类型

- [在trait定义中指定站位符](./doc/trait_plus.md)

- [默认泛型参数与运算符重载](./doc/trait_plus.md)
- [完全限定语法](./doc/trait_plus.md)

- [父trait](./doc/trait_plus.md)
- [newtype模式](./doc/trait_plus.md)

## 类型

- [类型别名](./doc/htype.md)

- [never type 介绍](./doc/htype.md)
- [动态大小类型和Sized trait](./doc/htype.md)

## 高级函数和闭包

- [函数指针](./doc/hfn.md)

- [返回闭包](./doc/hfn.md)

## 宏

- [宏介绍](./doc/macro.md)
- [声明宏 ](./doc/macro.md)
- [自定义derive宏](./doc/macro.md)
- [类属性宏和类函数宏](./doc/macro.md)
- [例子](./demo/macro)

## 实战

[rust-algorithms](https://github.com/EbTech/rust-algorithms)

[TheAlgorithms/Rust](https://github.com/TheAlgorithms/Rust)

