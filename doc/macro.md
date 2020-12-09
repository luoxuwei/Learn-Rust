# 宏介绍

1、Rust中的宏主要有两种，一种是使用macro_rules！的声明宏，一种是过程宏。而过程宏又主要分为三种： （1）自定义宏#[derive]，在结构体、枚举等上指定通过derive属性添加代码； 

（2）类属性宏，定义可用于任意项的自定义属性； 

（3）类函数宏，看起来像函数但是作用于作为参数传递的Token。 

2、宏和函数 

（1）宏是一种为写其它代码而写代码的方式。宏对于减少大量编写代码和维护代码非常有用。 

（2）一个函数标签必须声明函数参数个数和类型，宏只接受可变参数。 

（3）宏的定义比函数的定义更复杂。 

（4）在调用宏 之前 必须定义并将其引入作用域，而函数则可以在任何地方定义和调用。

# 声明宏

使用marco_rules！的声明宏

 例子1： 

 let v = vec![1, 2, 3];//vec！就是用marco_rules！的声明宏  

例子2： 

（1）mkdir learn_marco1 

（2）cd learn_marco1 

（3）vim Cargo.toml，编辑工作空间： 

 [workspace] 

members = [ 

"mac", 

"main",

 ]  

（4）cargo new mac --lib

（5）编辑mac/src/lib.rs: 

```rust
#[macro_export]
macro_rules! my_vec { 
  ( $( $x:expr ),* ) => { 
    { 
      let mut temp_vec = Vec::new(); 
      $( 
        temp_vec.push($x); 
      )* 
      temp_vec 
    } 
  }; 
}  
```

（6）cargo new main 

（7）编辑main/Cargo.toml添加：

  [dependencies]

 mac = {path = "../mac"}  

（8）编辑main/src/main.rs如下：

```rust
use mac; 
fn main() { 
  let v = mac::my_vec![1, 2, 3]; 
  println!("v = {:?}", v); 
  println!("Hello, world!"); 
}  
```

说明：在上面的例子中定义了声明宏，并在main中进行使用。宏中的代码等价于如下

```rust
let mut temp_vec = Vec::new(); 
temp_vec.push(1); 
temp_vec.push(2); 
temp_vec.push(3); 
temp_vec
```

# 过程宏

1、过程宏介绍 

过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。 

定义过程宏的函数接受一个 TokenStream 作为输入并产生一个 TokenStream 作为输出。这也就是宏的核心：宏所处理的源代码组成了输入 TokenStream，同时宏生成的代码是输出 TokenStream。如下： 

 use proc_macro;  

\#[some_attribute]  

pub fn some_name(input: TokenStream) -> TokenStream {  }  

2、自定义derive宏

 （1）mkdir learn_marco2 

（2）cargo new hello_macro --lib 

（3）cd hello_macro ，编辑src/lib.rs如下：

 pub trait HelloMacro { 

fn hello_macro(); 

}  

（4）cargo new hello_macro_derive --lib 

（5）编辑hello_macro_derive/Cargo.toml添加如下：

  [lib] 

proc-macro = true

 [dependencies] 

syn = "0.14.4" 

quote = "0.6.3"  

（6）编辑hello_macro_derive/src/lib.rs如下：  

```rust
extern crate proc_macro; 
use crate::proc_macro::TokenStream;
use quote::quote; 
use syn; 
#[proc_macro_derive(HelloMacro)] 
pub fn hello_macro_derive(input: TokenStream) -> TokenStream { 
  // 构建 Rust 代码所代表的语法树 
  // 以便可以进行操作 
  let ast = syn::parse(input).unwrap();//解析出DeriveInput结构体 
  // 构建 trait 实现 
  impl_hello_macro(&ast) 
} 
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream { 
  let name = &ast.ident; 
  let gen = quote! {
    impl HelloMacro for #name { 
      fn hello_macro() { 
        println!("Hello, Macro! My name is {}", stringify!(#name)); 
      } 
    } 
  }; 
  gen.into() 
}  
```

（7）使用宏：cd .. 

（8）cargo new pancakes 

（9）编辑pancakes/Cargo.toml如下： 

 [dependencies]

 hello_macro = { path = "../hello_macro" } 

hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }  

（10）编辑pancakes/src/main.rs如下：

```rust
use hello_macro::HelloMacro; 
use hello_macro_derive::HelloMacro; 
#[derive(HelloMacro)] 
struct Pancakes; 
fn main() { 
  Pancakes::hello_macro(); 
}  
```

 说明：在hello_macro_derive函数的实现中，syn 中的 parse_derive_input 函数获取一个 TokenStream 并返回一个表示解析出 Rust 代码的 DeriveInput 结构体（对应代码```syn::parse(input).unwrap();```）。该结构体相关的内容大体如下：

```rust
DeriveInput { 
  // --snip--  
  ident: Ident {  
    ident: "Pancakes",  
    span: #0 bytes(95..103) 
  },  
  data: Struct(  
    DataStruct {  
      struct_token: Struct,  
      fields: Unit,  
      semi_token: Some( Semi ) 
    } 
  )  
}
```

  3、类属性宏 

类属性宏与自定义派生宏相似，不同于为 derive 属性生成代码，它们允许你创建新的属性。 

```
例子：
可以创建一个名为 route 的属性用于注解 web 应用程序框架（web application framework）的函数：
#[route(GET, "/")]
fn index() {

#[route] 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

说明：类属性宏其它工作方式和自定义derive宏工作方式一致。


```

4、类函数宏
类函数宏定义看起来像函数调用的宏。类似于 macro_rules!，它们比函数更灵活。
例子：
如sql！宏，使用方式为：
let sql = sql!(SELECT * FROM posts WHERE id=1);
则其定义为：
\#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {

5、宏的资料推荐
https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html