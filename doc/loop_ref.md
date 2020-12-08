引用循环，让两个list指向彼此，例子代码如下： 

```rust
use std::rc::Rc; 
use std::cell::RefCell; 
use crate::List::{Cons, Nil}; 

#[derive(Debug)] 
enum List {   
  Cons(i32, RefCell<Rc<List>>),   
  Nil, 
} 
impl List {   
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {     
    match self {       
      Cons(_, item) => Some(item),       
      Nil => None,     
    }   
  } 
}  
fn main() {   
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));   
  println!("a initial rc count = {}", Rc::strong_count(&a));   
  println!("a next item = {:?}", a.tail());   
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));   
  println!("a rc count after b creation = {}", Rc::strong_count(&a));   
  println!("b initial rc count = {}", Rc::strong_count(&b));   
  println!("b next item = {:?}", b.tail());   
  if let Some(link) = a.tail() {     
    *link.borrow_mut() = Rc::clone(&b);   //将a修改为指向b   
  }   
  println!("b rc count after changing a = {}", Rc::strong_count(&b)); //输出引用计数，为2  
  println!("a rc count after changing a = {}", Rc::strong_count(&a)); //输出引用计数，为2   
  //下面的调用将出错，因为上面已经制造了循环引用，编译器无法找到tail   
  // println!("a next item = {:?}", a.tail()); 
}  
```

![Rust](https://cdn.learnku.com/uploads/images/202002/05/53818/xMGMxFw6zt.PNG!large) 

说明：

 （1）在main函数的结尾，Rust会尝试丢弃b，这会使a和b中Rc实例的引用计数减为1.但是，因为a仍然引用b中的Rc<List>，因此Rc<List>的计数是1而不是0，所以Rc<List>在堆上的内存不会被丢弃，从而造成内存泄露。 

（2）在上面的例子中，其相互引用关系如上图所示，如果打印a.tail()，那么就会在上面的循环中一直查找（符合（_, item）的情况），但是永远没有结束（因为是个循环，永远找不到匹配的情况），最终会造成栈溢出。

