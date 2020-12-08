# 使用外部包例子复习

可以通过发布包来向别人来分享代码。crates.io用来分发包的源代码，所以它主要托管开源代码。  

复习 

我们在前面使用rust-crypto时，直接在Cargo.toml中添加如下：  

[dependencies] 

rust-crypto = "0.2"  

通过此种方式使用别人的包rust-crypto，此包就是发布在crates.io上的。  

例子：

```rust
use crypto::digest::Digest; 
use crypto::sha3::Sha3; 
fn main() { 
  let mut hasher = Sha3::sha3_256(); 
  hasher.input_str("hello world"); 
  let hex = hasher.result_str(); 
  println!("sha: {}", hex); 
}
```

# 文档注释

编写有用的文档注释 

（1）在基础部分，我们讲解了代码注释，通过//来注释； 

（2）Rust也有特定的用于文档的注释类型，通常称为文档注释，它们会生成HTML文档。它们通过///来注释。

 例子： 通过cargo new mylib --lib 创建src/lib.rs  

src/lib.rs 

```rust
/// Adds one to the number given 
/// 
/// # Examples 
/// 
///  
/// let five = 5; 
/// 
/// assert_eq!(6, mylib::add_one(5)); 
///  
pub fn add_one(x: i32) -> i32 {
  x + 1 
} 
```

 运行cargo doc会生成这个文档注释的HTML文档。 

运行cargo doc --open会构建当前crate文档的HTML并在浏览器中打开。 

（3）哪些通常需要注释 

* Panics：这个函数可能会panic！的场景；

* Errors：如果该函数返回Result类型，此部分会描述会出现哪些错误； 

* Safety：如果这个函数使用unsafe代码，则应该说明。

(4）文档注释作为测试

 cargo test也会像文档中的示例代码那样进行测试。

 运行方式为：```cargo test ```

src/lib.rs 

```rust
/// Adds one to the number given 
/// 
/// # Examples 
/// 
///  
/// let five = 5; 
/// 
/// assert_eq!(6, mylib::add_one(5)); //运行cargo test，会进行此测试 
///  
pub fn add_one(x: i32) -> i32 { 
  x + 1 
}  
```

（5）为crate或者模块整体提供文档的注释：```//！``` 

例子：src/lib.rs  

```rust
//! My Crate 
//! 
//! 'my_crate' is a collection of utilites to make performing certain calculations more convenient 
//! 
/// Adds one to the number given 
/// 
/// # Examples 
/// 
///  
/// let five = 5; 
/// 
/// assert_eq!(6, mylib::add_one(5)); 
///  
pub fn add_one(x: i32) -> i32 {
  x + 1 
}  
```

查看效果：  cargo doc --open

# crate 的发布与撤回

crate的发布与撤回（此部分没有实际操作过） 

（1）创建Crates.io账号：通过Github账户注册，并通过cargo login ****** 来登陆 

（2）发布前需要在Cargo.toml中增加描述： 

 [package] 

name = "package_name" 

version = "0.1.0" 

license = "MIT" #[Linux 基金会 的 Software Package Data Exchange (SPDX)](http://spdx.org/licenses/) 列出了可以使用的标识符

authors = ["linghuyichong"]

description = "some thing descript the package"  

运行cargo publish来发布。

（3）撤回指定版本  

cargo yank --vers 0.1.0

# 文档注释（pub use 导出合适的公有 API）

使用pub use导出合适的公有API

 例子1：  

```rust
//! # Art 
//! 
//! 一个描述美术信息的库。 
pub mod kinds { 
/// 采用 RGB 色彩模式的主要颜色。 
  pub enum PrimaryColor { 
    Red, 
    Yellow, 
    Blue, 
  } 
/// 采用 RGB 色彩模式的次要颜色。 
  pub enum SecondaryColor { 
    Orange, 
    Green, 
    Purple, 
  } 
}  
pub mod utils { 
  use crate::kinds::*; 
/// 等量的混合两个主要颜色 
/// 来创建一个次要颜色。 
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor { 
    SecondaryColor::Orange 
  }
}   
//========================================
```

 例子2：

```rust
//! # Art 
//! 
//! 一个描述美术信息的库。 
//+++++以下为例子2添加部分+++++++++ 
pub use kinds::PrimaryColor; 
pub use kinds::SecondaryColor; 
pub use utils::mix; 
//+++++++++++++++++++++++++++++++ 
pub mod kinds { 
/// 采用 RGB 色彩模式的主要颜色。 
  pub enum PrimaryColor { 
    Red, 
    Yellow, 
    Blue, 
  } 
/// 采用 RGB 色彩模式的次要颜色。 
  pub enum SecondaryColor { 
    Orange, 
    Green, 
    Purple, 
  } 
}  
pub mod utils {
  use crate::kinds::*; 
/// 等量的混合两个主要颜色 
/// 来创建一个次要颜色。 
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor { 
    SecondaryColor::Orange 
  } 
} 
```

 运行cargo doc --open后观察两者的不同

