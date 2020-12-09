# 对象

1、面向对象的几个特性：对象、封装、继承。

 2、对象 

（1）面向对象的程序由对象组成。一个对象包含数据和操作这些数据的过程。 

（2）在Rust中，结构体和枚举类型通过impl块提供方法。 

例子：  

```rust
struct Dog { 
  name: String, 
} 
impl Dog { 
  fn print_name(&self) { 
    println!("dog name: {}", self.name); 
  }
} 
fn main() { 
  let d = Dog{name: String::from("wangcai")}; 
  d.print_name(); 
}
```

# 封装

1、封装 

（1）封装的思想：对象的实现细节不能被使用对象的代码获取到。 

（2）在Rust中，使用pub关键字来标记模块、类型、函数和方法是公有的，默认情况下一切都是私有的。

 例子：

 (1) mkdir learn_oo

 (2) cd learn_oo

 (3) 编辑Cargo.toml： 

 [workspace] members = [     

"getaver",     

"main",

 ] 

 (4) cargo new getaver --lib

 (5) 编辑 getaver/src/lib.rs，代码如下：  

```rust
pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect{
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.aver
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

```

 (6) cargo new main 

(7) 编辑main/Cargo.toml,添加：

  [dependencies] 

getaver = {path = "../getaver"}  

(8) 编辑main/src/main.rs,代码如下:
```rust
use getaver::AverCollect; 
fn main() {   
  let mut a = getaver::AverCollect::new();   
  a.add(1);   
  println!("average value = {}", a.average());   
  a.add(2);   
  println!("average value = {}", a.average());   
  a.add(3);   
  println!("average value = {}", a.average());   
  a.remove();   
  println!("average value = {}", a.average());   
  println!("Hello, world!"); 
} 
```

 2、继承 

Rust不支持继承。但是Rust可以通过trait进行行为共享。