# 类型别名

1、类型别名 

例子： 

```rust
type Kilometers = i32; 
let x: i32 = 5;  
let y: Kilometers = 5;  
println!("x + y = {}", x + y); 
```

 说明：例子中Kilmeters 是i32的同义词。Kilometers 类型的值完全当作i32类型来对待。 类型别名的主要用途是减少重复。 

（1）考虑如下类型：

```rust

//Box<dyn Fn() + Send + `static>
//如代码：  
let f: Box<dyn Fn()+Send+`static> = Box::new(|| println!("hi"));  
fn takes_long_type(f: Box<dyn Fn()+Send+`static>) {  
  // --snip--  
}  

fn returns_long_type() -> Box<dyn Fn()+Send+`static> {  
  // --snip--  
}  
//使用别名，代码：  
type Thunk = Box<dyn Fn()+Send+`static>;
let f: Thunk = Box::new(|| println!("hi"));  
fn takes_long_type(f: Thunk) {  
  // --snip-- 
}  

fn returns_long_type() -> Thunk {  
  // --snip--  
}  
```

（2）考虑如下例子：

```rust
use std::io::Error; //标准库中的std::io::Error结构体代表了所有可能的I/O错误  
use std::fmt;  
pub trait Write {  
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
  fn flush(&mut self) -> Result<(), Error>;  
  fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>; 
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;  
}  

//加上如下类型别名声明：  
type Result<T> = std::result::Result<T, std::io::Error>; //result<T, E> 中 E 放入了 std::io::Error  

//代码就可以变成：  
pub trait Write {  
  fn write(&mut self, buf: &[u8]) -> Result<usize>;  
  fn flush(&mut self) -> Result<()>;  
  fn write_all(&mut self, buf: &[u8]) -> Result<()>;  
  fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;  
}
```

# never type

1、从不返回的never type

 Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。我们更倾向于称之为 never type。在函数不返回的时候充当返回值：  

fn bar() -> ! {  

// --snip-- 

}  

例子1： 

//Cargo.toml文件  

[dependencies] 

rand = "0.6.0" 

 //src/main.rs 

```rust
use std::io;  
use std::cmp::Ordering;  
use rand::Rng;  
fn main() {  
  println!("Guess the number!");  
  let secret_number = rand::thread_rng().gen_range(1, 101); 
  loop {  
    println!("Please input your guess.");  
    let mut guess = String::new();  
    io::stdin().read_line(&mut guess).expect("Failed to read line");  
    let guess: u32 = match guess.trim().parse() {  
      Ok(num) => num,  
      Err(_) => continue, //continue 的值是 !。 
      //当 Rust 要计算 guess 的类型时，它查看这两个分支。 
      //前者是 u32 值，而后者是 ! 值。 
      //因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32 
    };  
    println!("You guessed: {}", guess);  
    match guess.cmp(&secret_number) {  
      Ordering::Less => println!("Too small!"),  
      Ordering::Greater => println!("Too big!"), 
      Ordering::Equal => {  
        println!("You win!");  
        break;  
      }  
    }  
  }  
}  
```

说明：never type 可以强转为任何其他类型。允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。 

例子2：panic! 

Option<>上的 unwrap 函数代码： 

```rust
impl<T> Option<T> {  
  pub fn unwrap(self) -> T {  
    match self {  
      Some(val) => val,  
      None => panic!("called `Option::unwrap()` on a `None` value"), 
    }  
  } 
}  
```

说明： 

match 时，Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。

# 动态大小类型

动态大小类型（dynamically sized types），有时被称为 “DST” 或 “unsized types”，这些类型允许我们处理只有在运行时才知道大小的类型。 

（1）str 是一个 DST；直到运行时我们都不知道字符串有多长。 

例子：//如下代码是错误的  

let s1: str = "Hello there!"; 

 let s2: str = "How's it going?";  

正确代码为：  

let s1: &str = "Hello there!";  

let s2: &str = "How's it going?";  

说明： 

&str 则是 两个 值：str 的地址和其长度。这样，&str 就有了一个在编译时可以知道的大小：它是 usize 长度的两倍。也就是说，无论字符串是多大，&str的大小我们总是知道的。

因此，引出动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。如：Box<str> 或 Rc<str>、&str等。

 另一个动态大小类型是trait。每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型。为了将 trait 用于 trait 对象，必须将他们放入指针之后，比如 &Trait 或 Box<Trait>（Rc<Trait>也可以）。 

（2） Sized trait 

为了处理 DST，Rust 用Sized trait 来决定一个类型的大小是否在编译时可知。这个 trait 自动为编译器在编译时就知道大小的类型实现。

 例子： 

```rust
//fn generic<T>(t: T) {//T为编译时就知道大小的类型
//    // --snip--
//}
//
////等价于
//fn generic<T: Sized>(t: T) {//T为编译时就知道大小的类型
//    // --snip--
//}
//
////如何放宽这个限制呢？Rust提供如下方式：
//fn generic<T: ?Sized>(t: &T) {//T 可能是Sized，也可能不是 Sized 的
//    // --snip--
//}

```





