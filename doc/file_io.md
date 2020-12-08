 问题描述 此问题来自于Rust社区，链接地址：https://users.rust-lang.org/t/why-must-i-mutably-borrow-struct-if-method-result-is-while-loop-condition/36310 

在问题中，题主使用如下代码读取文件内容Ｏｋ：  

```rust
//第一段代码 
use std::io::{self,BufRead}; 
use std::fs::File; 
fn main() -> std::result::Result<(), std::io::Error> {   
  let filename = "test.bed";   
  let fi = File::open(filename)?;   
  let bedreader = io::BufReader::new(fi); //注意，bedreader不是mutable   
  for line in bedreader.lines() {     
    println!("{}", line?);   
  }   
  Ok(()) 
}
```

 但是当换成如下代码后，出错：  

```rust
//第二段代码 
use std::io::{self,BufRead}; 
use std::fs::File; 
fn main() -> std::result::Result<(), std::io::Error> {   
  let filename = "test.bed";   
  let fi = File::open(filename)?;   
  let bedreader = io::BufReader::new(fi); //注意，bedreader不是mutable   
  let mut line = String::new();   
  while bedreader.read_line(&mut line).unwrap() > 0 {//此处报错     
    println!("{}", line.trim_end());     
    line.clear();   
  }   
  Ok(()) 
}
```

  在第二段代码中，题主认为试用while循环读取内容，line已经定义成mutable的了，不应该出错。 

 问题解决 

对于第二段代码，进行如下修改后就能正确运行，如下： 

```rust
//对第二段代码修正 
use std::io::{self,BufRead}; 
use std::fs::File; 
fn main() -> std::result::Result<(), std::io::Error> {   
  let filename = "test.bed";   
  let fi = File::open(filename)?;   
  let mut bedreader = io::BufReader::new(fi); //注意，bedreader是mutable   
  let mut line = String::new();   
  while bedreader.read_line(&mut line).unwrap() > 0 {//此处不再报错     
    println!("{}", line.trim_end());     
    line.clear();   
  }   
  Ok(()) 
}  
```

  问题解释     

​    在第一段代码中，在for循环中使用的是通过bedreader.lines()读取每一行，而该方法的定义为：

 fn lines(self) -> Lines<Self>

​        而在第二段代码中使用的read_line，定义为：

 fn read_line(&mut self, buf: &mut String) -> Result<usize>

​    因此，从两个方法的定义可以看出，第一段代码不需要定义成mutable,第二段代码中需要定义成mutable。

