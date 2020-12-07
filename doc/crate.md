1、定义 

（1）包：Cargo的一个功能，允许构建、测试和分享crate。 

（2）Crate：一个模块的树形结构，形成库或二进制项目。 

（3）模块：通过use来使用，用来控制作用域和路径的私有性。 

（4）路径：一个命名例如结构体、函数或模块等项的方式。  

2、包和Crate 

（1）crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块。 

（2）包提供一系列功能的一个或多个Crate。 

（3）Crate root是src/main.rs或者是src/lib.rs。 说明：如果只有main.rs则说明这个包只有一个crate（main），如果同时拥有main.rs和其它的lib.rs（不一定是这个名字）则说明拥有多个crate。

 （4）crate会将一个作用域的相关功能分组到一起，使得该功能可以很方便的在多个项目之间共享。 

 3、使用模块控制作用域和私有性 

（1）创建一个lib可以通过命令cargo new --lib libname来进行创建。 

（2）中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的，需要使用pub才能暴露给外部。 （3）创建模块，例如：  

```rust
//factory.rs 
mod refrigerator { //冰箱//需要使用 pub，否则别人无法使用 
  fn refrigeration() {//需要使用 pub，否则别人无法使用 
  } 
}  

mod washing_machine { //需要使用 pub，否则别人无法使用 
  fn wash() {//需要使用 pub，否则别人无法使用 
  } 
}   

//lib.rs 
pub mod factory;  

//main.rs 
use mylib::factory; 
fn main() { 
  factory::refrigerator::refrigeration(); 
  println!("Hello, world!");
} 
```

 知识点：需要使用pub，否则无法使用。 

 4、使用绝对路径和使用相对路径 

例子：  

```rust
mod A {
  pub mod B { 
    pub fn prin() {
      println!("++++++"); 
      super::ppp();//使用父路径 
    }
  }  
  
  pub fn ppp() { 
    println!("++++++ ppp"); 
  } 
} 

fn main() { 
  use A::B::prin; //相对路径 
  prin(); 
  A::B::prin();//绝对路径 
  println!("Hello, world!"); 
}  
```

 5、对结构体的使用  

```rust
mod modA { 
  #[derive(Debug)] 
  pub struct A { 
    pub number: i32, 
    name: String, 
  }  
  
  impl A {
    pub fn new_a() -> A {
      A { 
        number: 20, 
        name: String::from("A"), 
      } 
    }  
    
    pub fn print_a(&self) {
      println!("number = {}, name = {}", self.number, self.name); 
    }
  } 
}  

fn main() { 
  let a = modA::A::new_a(); 
  a.print_a(); 
  let n = a.name;//error，因为name是私有的 
  println!("Hello, world!");
}   
```

6、使用use和as，使用*引进所有 *

```rust
 //使用use 
use modA::A; 
let a = A::new_a();  
//使用as 
use modA::A as myA; 
let a = myA::new_a();  
//使用* 
use modA::*; 
let a = myA::new_a(); 
```

 7、使用pub use重导出 

 8、外部包 在Cargo.toml下的dependencies下 

（1）路径  

[dependencies] 

mylib = {path = "./mylib"}  

（2）github上的版本  

[dependencies] 

rust-crypto = "0.2" 

serde = "1.0.63" 

bincode = "1.2.0" 

serde_derive = "1.0.27"  

例子：  

```rust
//! SHA3-256 示例 
extern crate crypto; 
//extern crate rustc_hex; 
use self::crypto::digest::Digest; 
use self::crypto::sha3::Sha3; 
//use rustc_hex::{ToHex,FromHex}; 
fn main() { 
  // create a SHA3-256 object 
  let mut hasher = Sha3::sha3_256();  
  // write input message 
  hasher.input_str("hello world");  
  
  //read hash digest 
  let hex = hasher.result_str(); 
}
```

# crate 的发布与撤回

crate的发布与撤回（此部分没有实际操作过） 

（1）创建Crates.io账号：通过Github账户注册，并通过cargo login ****** 来登陆 

（2）发布前需要在Cargo.toml中增加描述： 

 [package] 

name = "package_name" 

version = "0.1.0" 

license = "MIT" #[Linux 基金会 的 Software Package Data Exchange (SPDX)](http://spdx.org/licenses/) 列出了可以使用的标识符 

authors = ["xxxx"] 

description = "some thing descript the package"  

运行cargo publish来发布。 

 （3）撤回指定版本  cargo yank --vers 0.1.0