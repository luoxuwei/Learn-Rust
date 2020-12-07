1.所有权是rust语言中最与众不同的特性，它让rust无需垃圾回收就可以保证内存安全。

2.计算机语言对内存的管理有三种方式： 

* 程序员自己分配和回收，例如：c/c++。 
*  语言自带GC，例如go语言。 
*  rust语言为第三种方式，通过所有权系统管理内存，编译器在编译时根据规则对内存使用进行检查。

3.栈和堆

- 栈的操作是后进先出，存储于栈上的数据编译时都占用已知固定的大小。分配方式：直接分配 

* 编译时大小未知或者是变化的数据在堆上进行存储。　表示方式：用指针和大小表示

4.变量作用域

一对花括号表示的范围，就是作用域，如下：

```rust
{

let i = 1;   
...                                                   |
let j = 2;

}
```

说明： 

* 变量进入它的作用域有效 
* 离开作用域后无效

5.对于String类型，如：

```rust
let s = String::from("hello"); 

s.push_str(", world");
```

对于s，其所需要内存大小是变化的，因此需要在堆上进行分配。 对于在堆上分配的内存，编程语言一般有几种回收方式： 

* 有GC的语言自动回收，如go、java 
*  程序员手动回收，如c/c++ 
* rust语言采用：当变量离开它的作用域后，就自动释放。

```rust
{

let s = String::from("hello");

...

} //自动释放s，s不再有效
```

这是一个将 String 需要的内存返回给操作系统的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

6.移动

例子1：

```rust
let x = 5;  //将5绑定到x

let y = x;  //生成一个x的拷贝绑定到y
```

由于都是确定的类型，所以在存放在栈中。

例子2:

```rust
let s1 = String::from("hello");  //s1在堆上

let s2 = s1;    //移动s1到s2，此后不能再使用s1

println!("{}, world!", s1); //此处要报错，s1不能再使用
```

原因： 

* 两者的存放方式，是浅拷贝
*  如果s1继续有效，那么释放的时候（调用drop）会释放两次，会发生错误，因此这里s1不能再使用
*  **说明**：rust语言默认是浅拷贝，同时将被拷贝的变量就会变得无效。

7.克隆

如果要复制堆上的内容，则使用clone，例子：

```rust
let s1 = String::from("hello");

let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

8.在栈上的数据拷贝

例如:

```rust
let x = 5;

let y = x;

println!("x = {}, y = {}", x, y);  //ok，x，y都在栈上
```

Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。

常用的具有Copy trait的有： 

* 所有整数类型，比如 u32。 
* 布尔类型，bool，它的值是 true 和 false。 
* 所有浮点数类型，比如 f64。 
* 字符类型，char。 
*  元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

9.函数与作用域

例子1:

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                                 // ... 所以到这里不再有效
    let x = 5;                              // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，                                                           // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

例子2:

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值,移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到takes_and_gives_back 中, 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃
fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```

**总结：**将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

```rust
//1 rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//
//2 堆和栈
//编译的时候数据的类型大小是固定的，就是分配在栈上的
//编译的时候数据类型大小不固定，就是分配堆上的
//
//3 作用域:{}
//
//4 String内存回收
//5 移动
//6 clone
//7 栈上数据拷贝
//8 函数和作用域
fn main() {
    let x: i32 = 1;

    {
        let y: i32 = 1;
        println!("x = {}", x);
        println!("y = {}", y);
    }

    {
        let s1 = String::from("hello");
        //s1.push_str(" world");
        println!("s1 = {}", s1); //String类型离开作用域的时候会调用drop方法

        let s2 = s1; 
        println!("s2= {}", s2);
        //println!("s1= {}", s1); //move to s2, s1 invalid
        //
        //clone
        let s3 = s2.clone();
        println!("s3= {}", s3);
        println!("s2= {}", s2);
    }

    //copy trait
    let a = 1;
    let b = a;
    println!("a = {}, b = {}", a, b);
    //常用的具有copy trait有：
    //所有的整型
    //浮点型
    //布尔值
    //字符类型 char
    //元组
    println!("Hello, world!");
}
```



