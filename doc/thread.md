# 线程介绍

相关概念 

（1）进程是资源分配的最小单位，线程是CPU调度的最小单位。 

（2）在使用多线程时，经常会遇到的一些问题： 

* 竞争状态：多个线程以不一致的顺序访问数据或资源；
* 死锁：两个线程相互等待对方停止使用其所拥有的资源，造成两者都永久等待； 
* 只会发生在特定情况下且难以稳定重现和修复的bug 

（3）编程语言提供的线程叫做绿色线程，如go语言，在底层实现了M:N的模型，即M个绿色线程对应N个OS线程。但是，Rust标准库只提供1：1的线程模型的实现，即一个Rust线程对应一个Os线程。 

运行时代表二进制文件中包含的由语言本身提供的代码，这些代码根据语言的不同可大可小，不过非汇编语言都会有一定数量的运行时代码。通常，大家说一个语言“没有运行时”，是指这个语言的“运行时”很小。Rust、C都是几乎没有运行时的。

# 创建线程与等待线程结束

1、创建线程 

（1）方法：调用thread::spawn即可 

（2）例子：  

```rust
use std::thread; 
use std::time::Duration; 
fn main() {   
  thread::spawn(||{     
    for i in 1..10 {       
    println!("hi number {} from the spawned thread!", i);                     thread::sleep(Duration::from_millis(1));     
    }   
  });   
  for i in 1..5 {     
    println!("hi number {} from the main thread!", i);                       thread::sleep(Duration::from_millis(1));   
}
}
```

  2、等待线程结束 

上面的例子中，在主线程结束之前并不能保证所有子线程都结束了（主线程提前结束，就不能保证这些子线程都执行了），因此通过以下方法等待子线程结束。 

（1）方法：join 

（2）例子1：  

```rust
use std::thread; 
use std::time::Duration; 
fn main() {   
  let handle = thread::spawn(||{     
    for i in 1..10 {       
      println!("hi number {} from the spawned thread!", i);       thread::sleep(Duration::from_millis(1));     
    }   
  });   
  for i in 1..5 {     
    println!("hi number {} from the main thread!", i);     thread::sleep(Duration::from_millis(1));   
  }   
  handle.join().unwrap(); 
}  
```

（3）例子2：  

```rust
use std::thread; 
use std::time::Duration; 
fn main() {   
  let handle = thread::spawn(||{     
    for i in 1..10 {       
      println!("hi number {} from the spawned thread!", i);       thread::sleep(Duration::from_millis(1));     
    }   
  });   
  handle.join().unwrap();   
  for i in 1..5 {     
    println!("hi number {} from the main thread!", i);     thread::sleep(Duration::from_millis(1));   
  } 
}
```

# 线程与 move 闭包

线程与move闭包 

（1）错误例子：  

```rust
use std::thread; 
fn main() {   
  let v = vec![1, 2, 3];   
  let handle = thread::spawn(|| {     
    println!("Here's a vector: {:?}", v);   
  });   
  handle.join().unwrap(); 
}  
```

报错原因：Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。

 补充说明，如下例子： 

```rust
use std::thread; 
fn main() {   
  let v = vec![1, 2, 3];   
  let handle = thread::spawn(|| {     
    println!("Here's a vector: {:?}", v);   
  });   
  drop(v); // 错误：子线程可能还在运行，在使用v，但是此处已经将v drop掉了   
  handle.join().unwrap(); 
}  
```

（2）正确方法，使用move： 

```rust
use std::thread; 
fn main() {   
  let v = vec![1, 2, 3];   
  let handle = thread::spawn(move || {     
    println!("Here's a vector: {:?}", v);   
  });   
  handle.join().unwrap(); 
} 
```

 通过move，将v的所有权就移到子线程中了，主线程将不再使用v。 

如下，错误例子： 

```rust
use std::thread; 
fn main() {   
  let v = vec![1, 2, 3];   
  let handle = thread::spawn(move || {     
    println!("Here's a vector: {:?}", v);   });   
  drop(v); //错误，v已经被移动到子线程中了   
  handle.join().unwrap();
} 
```