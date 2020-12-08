# Box 介绍

1、最简单最直接的智能指针是box，其类型为Box<T> . box允许将值放在堆上而不是栈上，留着栈上的则是指向堆数据的指针。除了数据被存储在堆上外，box没有任何性能损失。  

2、box适合用于如下场景：

* 当有一个在编译时未知大小的类型，而又需要再确切大小的上下文中使用这个类型值的时候；（举例子：在一个list环境下，存放数据，但是每个元素的大小在编译时又不确定） 

* 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候；

* 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型时。 

3、使用Box在堆上存储数据：  

fn main（） {

 let b = Box::new(5); //此时5存储在堆上而不是栈上，b本身存储于栈上

 println!("b = {}", b); //离开作用域时同时清楚堆和栈上的数据 

}

# 使用实例

box使用的第一种场景，实例。 

（1）错误程序：  

```rust
enum List { 
  Cons(i32, List), //链表，类似于c语言的结构体定义： 
  //struct List{ 
  //int， 
  //struct List L；//当然是错误写法，c编译器此时不知道L有多大，正确写法应该是：Struct List *p；
  //} 
  Nil, 
} 
fn main() { 
  use List::Cons; 
  let list = Cons(1, Cons(2, Cons(3, Nil))); //要报错，因为编译器不知道给list分配多大的内存，类似于上面c语言那个错误写法 
  println!("Hello, world!"); 
}  
```

（2）正确的方式使用

```rust
Box  enum List { 
  Cons(i32, Box<List>), //用Box就把它变成了一个指针，类似于c语言的结构体定义： 
  //struct List{ 
  //int， 
  //struct List *p； 
  //} 
  Nil, 
} 
fn main() { 
  use List::Cons; 
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); 
  println!("Hello, world!"); 
}
```





