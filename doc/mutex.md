# 互斥器介绍

1、任何编程语言中的通道都类似于单所有权的方式，即一旦一个值传送到通道中，（发送者）将无法再使用这个值；而共享内存就类似于多所有权，即多个线程可以同时访问相同的内存位置。 

2、互斥器 

（1）作用：任意时刻，只允许一个线程访问某些数据； 

（2）互斥器的使用：使用前需要获取锁；使用后需要解锁数据。

 3、Rust中互斥器API：Mutex

```rust
use std::sync::Mutex; 
fn main() {   
  let m = Mutex::new(5);   
  {     
    let mut num = m.lock().unwrap();     
    *num = 6;   
  } //离开作用域，Mutex<> 的锁会自动释放   
  println!("m = {:?}", m); 
} 
```

 说明： 

（1）Mutex<>是一个智能指针，更准确的说，lock调用返回一个叫做MutexGuard的智能指针； 

（2）内部提供了Drop方法，实现当MutexGuard离开作用域时自动释放锁。

# 互斥器示例

1、线程间共享Mutex

（1）错误例子：  

```rust
use std::sync::Mutex; 
use std::thread; 
fn main() {   
  let counter = Mutex::new(0);   
  let mut handles = vec![];   
  for _ in 0..10 {     
    let handle = thread::spawn(move || {       
      let mut num = counter.lock().unwrap();       
      *num += 1;     
    });     
    handles.push(handle);   
  }   
  for handle in handles {     
    handle.join().unwrap();   
  }   
  println!("Result: {}", *counter.lock().unwrap()); 
}
```

  错误原因：不能将counter锁的所有权移动到多个线程中。 

（2）错误例子2：通过Rc<>来创建引用计数的值  

```rust
use std::rc::Rc; 
use std::sync::Mutex; 
use std::thread; 
fn main() {   
  let counter = Rc::new(Mutex::new(0));   
  let mut handles = vec![];   
  for _ in 0..10 {     
    let counter = Rc::clone(&counter);     
    let handle = thread::spawn(move || {       
      let mut num = counter.lock().unwrap();       
      *num += 1;     
    });     
    handles.push(handle);   
  }   
  for handle in handles {     
    handle.join().unwrap();   
  }   
  println!("Result: {}", *counter.lock().unwrap()); 
}  
```

错误原因：Rc<>不是线程安全的 

（3）例子3：使用Arc<>

说明: Arc<>是一个类似于Rc<>并可以安全的用于并发环境的类型，代码如下：

```rust
use std::sync::{Mutex, Arc}; 
use std::thread; 
fn main() {   
  let counter = Arc::new(Mutex::new(0));   
  let mut handles = vec![];   
  for _ in 0..10 {     
    let counter = Arc::clone(&counter);     
    let handle = thread::spawn(move || {       
      let mut num = counter.lock().unwrap();       
      *num += 1;     
    });     
    handles.push(handle);   
  }   
  for handle in handles {     
    handle.join().unwrap();   
  }   
  println!("Result: {}", *counter.lock().unwrap());
}  
```

2、RefCell<>/Rc<>与 Mutex<>/Arc<>的相似性 

（1）Mutex<>提供内部可变性，类似于RefCell<>； 

（2）RefCell<>/Rc<>是非线程安全的，而Mutex<>/Arc<>是线程安全的。

