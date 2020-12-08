# 解引用介绍

1、实现Deref trait允许我们重载解引用运算符。意思就是，如果A实现了Deref trait，那么就可以写如下代码： 

```rust
let a: A = A::new(); 
let b = &a; 
let c = *b;//对A类型解引用  
```

2、通过解引用使用指针的值。 

例子： 

 fn main() {

 let x = 5;

 let y = &x; 

assert_eq!(5, x); 

assert_eq!(5, *y); //解引用 

}  

3、像引用一样使用Box

例子： 

 fn main() {

 let x = 5;

 let y = Box::new(x); 

assert_eq!(5, x); 

assert_eq!(5, *y);

 }

# 解引用多态

```rust
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
//1.解引用强制多态
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    //此处解引用时，强制多态，将&String变为&str，否则的话此处需要：hello(&(*m)[..]); 
    hello(&m); //将MyBox变为&String，再将String的解引用，变为字符串slice。  &str
    println!("Hello, world!");
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

//2.解引用多态与可变性交互：
//解引用多态有如下三种情况
//（1）当T：Deref<Target=U>时，从&T到&U
//(2)当T：DerefMut<Target=U>时，从&mut T 到&mut U
//（3）当T：Deref<Target=U>时，从&mut T到&U
```

