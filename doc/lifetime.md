# 生命周期介绍

1、Rust中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样。

2、生命周期的主要目标是避免悬垂引用。

```rust
fn main() {
    ////错误例子1,r为悬垂引用
    //let r;                       //---------------------------+------------'a
    //{                            //                           +
    //    let x = 5;               //-------+------'b           |
    //    r = &x;                  //       |                   |
    //}                            //-------+                   |
    //println!("r = {}", r);       //                           |
    //                             //                           |
    //println!("Hello, world!");   //                           |
    //                             //---------------------------+
    //
    let r;                       //---------------------------+------------'a
                                 //                           +
    let x = 5;                   //-------+------'b           |
    r = &x;                      //       |                   |
                                 //       |                   |
    println!("r = {}", r);       //       |                   |
                                 //       |                   |
    println!("Hello, world!");   //       |                   |
}                                //------------------
```

3、Rust编译器使用借用检查器来检查生命周期是否有效。 错误例子：

```rust
{
  let r; //-------------------------------------+-------'a
  { //                                           |
    let x = 5; //-----+---'b                     |
    r = &x; //        |                          |
  } //----------------+                          |
  println!("r = {}", r); //r为悬垂引用             |
} //---------------------------------------------+
```

说明：r的生命周期为‘a，x的生命周期为’b，'b < 'a，被引用的对象比它的引用者存在的时间还短，那么必然报错。

正确例子：  

```rust
{ 
  let x = 5; //----------------------------------+-----------'b 
  let r = &x; //----------------+---'a           | 
  println!("r = {}", r); //     |                | 
} //----------------------------+----------------+ 
```

说明：数据比引用的生命周期更长，则是有效的引用

# 函数和结构体中的生命周期

1、函数中的泛型生命周期 

错误例子： 

```rust
fn longest(x: &str, y: &str) -> &str { 
  if x.len() > y.len() {
    x 
  } else { 
    y 
  } 
} 
fn main() { 
  let s1 = String::from("abcd"); 
  let s2 = String::from("ad"); 
  let r = longest(s1.as_str(), s2.as_str()); 
}
```

错误原因：rust的借用检查器不知道x和y的生命周期和返回值的生命周期是如何相关联的。

 生命周期的语法： ’a

 将上述函数修正： 

```rust
fn longest<’a>(x: &‘a str, y: &’a str) -> &‘a str { 
  if x.len() > y.len() {
    x 
  } else {
    y 
  } 
}  //上述函数中，所有的引用拥有相同的生命周期。当不遵守这个规则的参数传入时，检查器就会报错。
```

 2、注意的点 

（1）例子：  

fn longest<'a>(x: &‘a str, y: &str) -> &‘a str { x }  

如果返回值总是x，则不需要为y指定生命周期 

（2）当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用没有指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用。 

错误例子：  

```rust
fn a_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    let r = String::from("abc");
    r.as_str()//返回悬垂引用，报错
}
```

 3、结构体中的生命周期。包含引用的结构体，需要为结构体定义中添加生命周期注解。

```rust
#[derive(Debug)] 
struct A<'a> { 
  name: &'a str, 
} 
fn main() { 
  let n = String::from("andy"); 
  let a = A{ name: &n}; 
  println!("{:#?}", a); 
}
```

# 生命周期省略

生命周期省略 

例如：  

fn get_s(s: &str) -> &str {   s }  

（1）没有生命周期注解却能够编译，原因：早期的rust中必须显式的声明生命周期，后来rust团队将很明确的模式进行了注解的简化。 

（2）遵守生命周期省略规则的情况下能明确变量的声明周期，则无需明确指定生命周期。函数或者方法的参数的生命周期称为输入生命周期，而返回值的生命周期称为输出生命周期。 

（3）编译器采用三条规则判断引用何时不需要生命周期注解，当编译器检查完这三条规则后仍然不能计算出引用的生命周期，则会停止并生成错误。 

（4）生命周期注解省略规则适用于fn定义以及impl块定义，如下：   

a、每个引用的参数都有它自己的生命周期参数。例如如下：     

​     一个引用参数的函数，其中有一个生命周期：  

​        fn foo<'a>(x: &'a i32)      

​    两个引用参数的函数，则有两个生命周期 ： 

​        fn foo<'a, 'b>(x: &'a i32, y: &'b i32)      

以此类推。   

b、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：       

​    fn foo(x: &i32) -> &i32  等价于  fn foo<'a>(x: &'a i32) -> &'a i32   

 c、如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self或者&mut self，那么self的生命周期被赋予所有输出生命周期参数。例子在下面来看。

fn function(&self, x: &str, y: &str, ....) -> &str 

# 方法定义中的生命周期注解和静态生命周期

1、方法定义中的生命周期注解 

（1）结构体字段的生命周期必须总是在impl关键字之后声明并在结构体名称之后使用，因为这些声明周期是结构体类型的一部分。 

```rust
//例子1:
struct StuA<'a> { 
  name: &'a str, 
} 
impl<'a> StuA<'a> { 
  fn do_something(&self) -> i32 { 3 }
}  
//例子2：  
struct StuA<'a> { 
  name: &'a str, 
} 
impl<'a> StuA<'a> { 
  fn do_something2(&self， s: &str) -> &str { //此处符合声明周期注解省略的第三条规则 
    self.name 
  } 
}  
//例子3：  
struct StuA<'a> { 
  name: &'a str, 
} 
impl<'a> StuA<'a> { 
  fn do_something2<'b>(&self, s: &'b str) -> &'b str { 
    //self.name 
    s 
  } 
} 
fn main() { 
  let s = String::from("andy"); 
  let a = StuA {name: &s}; 
  let s1 = String::from("Andy"); 
  let ss = a.do_something2(&s1); 
  println!("ss = {}", ss); 
  println!("Hello, world!"); 
} 
```

 2、静态生命周期 

定义方式：'static，其生命周期存活于整个程序期间。所有的字符字面值都拥有’static生命周期，我们可以如下来标注：

  let s: &'static str = "I have a static filetime";

# 为带有生命周期标注的 struct 实现 trait 的问题

问题描述 问题来自于Rust中文社区，链接：https://rust.cc/article?id=c8c9e40a-b27c-4cf9-b344-1770a90d16fc　，错误代码如下 :

```rust
use std::str::FromStr;  
pub struct Wrapper<'a>(&'a str);  
impl FromStr for Wrapper<'_> {   
  type Err = ();      
  fn from_str(s: &str) -> Result { 
    Ok(Wrapper(s))   
  } 
}  
```

原因分析 在《Rust程序设计语言》中第10.3节中，讲到　“**编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。**”

 第一条规则是每一个是引用的参数都有它自己的生命周期参数。 

第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。 

第三条规则是如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋给所有输出生命周期参数。第三条规则使得方法更容易读写，因为只需更少的符号。 

在上述例子中，fn from_str函数显然是符合第二条规则，也就是说入参s: &str的生命周期被赋予为输出的生命周期。但是，输出参数中的Self对应的类型为结构体Wrapper，而Wrapper是有生命周期的限制的，此时编译器不知道如何判断，因此报错。 

问题题主的原因是要为自定义类型实现FromStr trait，但是此处很显然是不可以的。

# 生命周期的例子

结合泛型参数、trait bounds和生命周期  

```rust
use std::fmt::Display; 
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display { 
  println!("Announcement! {}", ann); 
  if x.len() > y.len() { 
    x 
  } else {
    y 
  } 
} 
fn main() { 
  let s1 = String::from("s1"); 
  let s2 = String::from("s2!"); 
  let ann = 128; 
  let r = longest_with_an_announcement(s1.as_str(), s2.as_str(), ann); 
  println!("r = {}", r); 
  println!("Hello, world!"); 
}
```





