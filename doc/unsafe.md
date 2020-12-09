# 不安全 Rust 介绍

1、在此节之前讨论过的都是安全的Rust，即Rust在编译时会强制执行的内存安全保证。不会强制执行这类内存安全保证的，就是不安全的Rust。

 2、不安全的Rust存在的两大原因： 

（1）静态分析本质上是保守的，就意味着某些代码可能是合法的，但是Rust也会拒绝。在此情况下，可以使用不安全的代码。 

（2）底层计算机硬件固有的不安全性。如果Rust不允许进行不安全的操作，有些任务根本就完成不了。

 3、不安全的Rust具有的超级力量 

Rust会通过unsafe关键字切换到不安全的Rust。不安全的Rust具有以下超级力量： 

（1）解引用裸指针 

（2）调用不安全的函数或者方法 

（3）访问或修改可变静态变量 

（4）实现不安全的trait 

注意：unsafe并不会关闭借用检查器或禁用任何其它的Rust安全检查规则，它只提供上述几个不被编译器检查内存安全的功能。unsafe也不意味着块中的代码一定就是不ok的，它只是表示由程序员来确保安全。

# 解引用裸指针

裸指针是可变和不可变的，分别写作*const T和*mut T。此处的星号不是解引用运算符，而是类型名称的一部分。 裸指针： 

（1）允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针 

（2）不保证指向有效的内存 

（3）允许为空 

（4）不能实现任何自动清理功能 

创建不可变和可变裸指针，可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针。

 例子：  

```rust
fn main() {   
  let mut num = 5;   
  let r1 = &num as *const i32;   
  let r2 = &mut num as *mut i32;   
  let address = 0x012345usize;   
  let _r = address as *const i32;   
  unsafe {     
    println!("r1 is: {}", *r1);     
    println!("r2 is: {}", *r2);   
  } 
} 
```

 说明：创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。在上述例子中，创建num的可变和不可变裸指针是可以编译的，但是如果创建可变和不可变引用编译不过（安全的Rust）。所以是使用裸指针需要非常注意。

# 调用不安全的函数或方法

（1）调用方式。不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 unsafe。

 例子1： 

```rust
unsafe fn dangerous() {
    println!("do something dangerous");
}

fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
    }
}

fn main() {
    unsafe {
        dangerous();
    }
    //dangerous(); //error

    foo();
    println!("Hello, world!");
}
```

（2）创建不安全代码的安全抽象  

```rust
fn foo() {   
  let mut num = 5;   
  let r1 = &num as *const i32;   
  let r2 = &mut num as *mut i32;   
  unsafe {     
    println!("r1 is: {}", *r1);     
    println!("r2 is: {}", *r2);   
  } 
} 

fn main() {   
  foo(); 
}  
```

（3）使用extern函数调用外部代码 

extern关键字，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）。

 例子1：调用c语言函数 

```rust
extern "C" {   
  fn abs(input: i32) -> i32; 
} 

fn main() {   
  unsafe {     
    println!("abs(-3): {}", abs(-3));   
  } 
} 
```

 例子2：c语言调用rust语言 

（a）cargo new foo --lib 

（b）vim src/lib.rs编写代码： 

```rust
#![crate_type = "staticlib"] 
#[no_mangle] 
pub extern fn foo(){   
  println!("use rust");
} 
```

 编写cargo.toml 

 [lib]

 name = "foo" 

crate-type = ["staticlib"]  

（c）编译后生成 libfoo.a的静态库 

（d）编写c语言的代码： 

 //main.c 

\#include <stdint.h>

\#include <stdio.h>

extern void foo();

 int main() {  

 foo();   

return 0;

 }  

（e）编译运行：gcc -o main main.c libfoo.a -lpthread -ldl

# 访问或者修改可变静态变量

（1）全局变量在 Rust 中被称为 静态（static）变量。 

例子：  

static HELLO_WORLD: &str = "Hello, world!"; 

fn main() {   

println!("name is: {}", HELLO_WORLD); 

}  

说明：通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法，并 必须 标注变量的类型，在这个例子中是 &'static str。静态变量只能储存拥有 'static 生命周期的引用。 

（2）常量和静态变量的区别： 

a、静态变量中的值有一个固定的内存地址（使用这个值总会访问相同的地址），常量则允许在任何被用到的时候复制其数据。

 b、静态变量可以是可变的，虽然这可能是不安全的（所以要用unsafe）。

 例子：  

```rust
//读取或修改一个可变静态变量 
static mut COUNTER: u32 = 0; 
fn add_to_count(inc: u32) {   
  unsafe {     
    COUNTER += inc;   
  } 
} 

fn main() {   
  add_to_count(3);   
  unsafe {     
    println!("COUNTER: {}", COUNTER);   
  } 
}
```

# 实现不安全的 trait

（1）当至少有一个方法中包含编译器不能验证的不变量时，该 trait 是不安全的； 

（2）在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe。

 例子： 

```rust
struct Bar();
unsafe trait Foo {   
  fn foo(&self); 
} 

unsafe impl Foo for Bar{   
  fn foo(&self) {     
    println!("foo");   
  } 
} 
fn main() {   
  let a: Bar = Bar();   
  a.foo();   
  println!("Hello, world!"); 
}
```









