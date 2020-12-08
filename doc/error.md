## 错误处理

1、rust语言将错误分为两个类别：可恢复错误和不可恢复错误 

（1）可恢复错误通常代表向用户报告错误和重试操作是合理的情况，例如未找到文件。rust中使用Result

来实现。 

（2）不可恢复错误是bug的同义词，如尝试访问超过数组结尾的位置。rust中通过panic！来实现。

2、panic！ 

fn main() { panic!("crash and burn"); }

3、使用BACKTRACE 例子：

 fn main() { let v = vec![1, 2, 3]; v[99]; }

运行时：RUST_BACKTRACE=1（任何不为0的值即可） cargo run，会打印出完整的堆栈。

4、Result

原型： 

enum Result

 { Ok(T), Err(E), }

使用例子：

```rust
use std::fs::File; 
fn main() { 
  let f = File::open("hello.txt"); 
  let f = match f { 
    Ok(file) => file, 
    Err(error) => { 
      panic!("Problem opening the file: {:?}", error) 
    }, 
  }； 
} 
```

 使用match匹配不同的错误： 

```rust
use std::fs::File; 
fn main() { 
  let f = File::open("hello.txt"); 
  let f = match f { 
    Ok(file) => file, 
    Err(error) => match error.kind() {
      ErrorKing::NotFound => println!("Not found!"), 
      _ =>panic!("Problem opening the file: {:?}", error), 
    }, 
  }； 
}  
```

5、失败时的简写 

（1）unwrap 例子：

```rust
use std::fs::File; 
fn main() { 
  let f = File::open("hello.txt").unwrap(); 
} 
```

 (2)expect 例子： 

```rust
use std::fs::File; 
fn main() { 
  let f = File::open("hello.txt").expect("Failed to open hello.txt"); 
}
```

# 传播错误

1、当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者，让调用者决定如何处理，这被称为传播错误。 例子：

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

```

2、传播错误的简写方式，提倡的方式 

```rust
use std::io; 
use std::io::Read; 
use std::fs::File; 
fn read_username_from_file() -> Result
 {   
   let mut f = File::open("hello.txt")?;   
   let mut s = String::new();   
   f.read_to_string(&mut s)?;   
   Ok(s) 
} 
```

 3、更进一步的简写 

```rust
use std::io; 
use std::io::Read; 
use std::fs::File; 
fn read_username_from_file() -> Result
 {   
   let mut s = String::new();   
   File::open("hello.txt")?.read_to_string(&mut s)?;   
   Ok(s) 
}
//说明1：rust提供了fs::read_to_string函数 
use std::io; 
use std::fs; 
fn read_username_from_file() -> Result
 {   
   fs::read_to_string("hello.txt") 
} 
//说明2：？运算符被用于返回Result的函数，如果不是返回Result的函数，用？会报错  
```



3、什么时候用panic！，什么时候用Result 

（1）示例、代码原型和测试适合panic，也就是直接panic！、unwrap、expect的方式 

（2）实际项目中应该多使用Result 

4、Option和Result Option是可能存在空值，而Result是可能存在错误

