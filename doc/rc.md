# 使用 Rc 引用计数智能指针

1、考虑需求 ![Rust](https://cdn.learnku.com/uploads/images/202002/02/53818/jyMGMbZdJ6.PNG!large) b和c共享a的所有权，该如何实现.

 例子1： 使用Box实现  

```rust
enum List {   
  Cons(i32, Box<List>),   
  Nil, 
} 
use crate::List::{Cons, Nil}; 
fn main() {   
  let a = Cons(5,
    Box::new(Cons(10,
      Box::new(Nil))));   
  let b = Cons(3, Box::new(a));   
  let c = Cons(4, Box::new(a)); 
} 
```

 此实现报错。提示已经被move了。

 例子2：使用Rc实现  

```rust
enum List {
  Cons(i32, Rc<List>),   
  Nil, 
} 
use crate::List::{Cons, Nil}; 
use std::rc::Rc; 
fn main() {   
  let a = Rc::new(Cons(5,Rc::new(Cons(10, Rc::new(Nil)))));   
  let b = Cons(3, Rc::clone(&a));  //此处使用a.clone()也是ok的  
  let c = Cons(4, Rc::clone(&a)); 
}  
```

2、克隆Rc<T>会增加引用计数  

```rust
fn main() {   
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));   
  println!("count after creating a = {}", Rc::strong_count(&a));   
  let b = Cons(3, Rc::clone(&a));   
  println!("count after creating b = {}", Rc::strong_count(&a)); 
  {     
    let c = Cons(4, Rc::clone(&a));     
    println!("count after creating c = {}", Rc::strong_count(&a));   
  }   
  println!("count after c goes out of scope = {}", Rc::strong_count(&a)); 
}  
```

通过Rc::strong_count函数能打印除引用计数。

 3、通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会造成数据竞争和不一致。

