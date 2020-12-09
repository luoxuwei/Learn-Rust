# 函数指针

1、函数指针

 函数指针允许我们使用函数作为另一个函数的参数。函数的类型是 fn ，fn 被称为 函数指针。

指定参数为函数指针的语法类似于闭包。 

例子：

```rust
fn add_one(x: i32) -> i32 {  
  x + 1  
}  
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {  
  f(arg) + f(arg)  
}  

fn main() {  
  let answer = do_twice(add_one, 5);  
  println!("The answer is: {}", answer);  
} 
```

 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。 

例子：//使用闭包和函数指针作为参数  

```rust
fn wrapper_func<T>(t: T, v: i32) -> i32 where T: Fn(i32) -> i32 { 
  t(v) 
} 
fn func(v: i32) -> i32 { 
  v + 1 
} 

fn main() { 
  let a = wrapper_func(|x| x+1, 1); 
  println!("a = {}", a); 
  let b = wrapper_func(func, 1); 
  println!("b = {}", b); 
}
```

# 返回闭包

返回闭包 

```rust
//错误例子：  
fn returns_closure() -> Fn(i32) -> i32 {
  |x| x + 1 
}  

//正确例子：  
fn returns_closure() -> Box<dyn Fn(i32)->i32> { 
  Box::new(|x| x + 1) 
} 

fn main() { 
  let c = returns_closure(); 
  println!("r = {}", c(1)); 
  //等价于println!("r = {}", (*c)(1)); 
  //解引用多态 
  println!("Hello, world!");
}
```





