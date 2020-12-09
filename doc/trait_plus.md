# 在trait中定义占位符

1、关联类型在trait定义中指定占位符类型 

关联类型是一个将类型占位符与trait相关论的方式。trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。如此可以定义一个使用多种类型的 trait。 

```rust
//（1）回忆使用关联类型的例子中，Iterator trait： 
pub trait Iterator {   
  type Item;  
  fn next(&mut self) -> Option;
}  

//（2）为什么不像如下，使用泛型？  
pub trait Iterator<T> {   
  fn next(&mut self) -> Option;
}  
//如果使用泛型，实现的例子如下： 
trait Iterator1<T> {  
  fn next(&mut self) -> Option<T>; 
} 

struct A {   
  value: i32, 
} 

impl Iterator1<i32> for A {   
  fn next(&mut self) -> Option<i32> {     
    println!("in next function: i32");     
    if self.value > 3 {       
      self.value += 1;       
      Some(self.value)     
    } else {       
      None     
    }   
  } 
} 

impl Iterator1<String> for A {   
  fn next(&mut self) -> Option<String> {     
    println!("in next function: string");     
    if self.value > 3 {       
      let s = String::from("hello");       
      Some(s)     
    } else {       
      None     
    }   
  } 
} 

fn main() {   
  let mut a = A{value: 3};   
  //a.next();  //错误，因为编译器不知道调用哪个实现  
  <A as Iterator1<String>>::next(&mut a);  //完全限定语法，带上了具体的类型   
  <A as Iterator1<i32>>::next(&mut a); //完全限定语法，带上了具体的类型   
  println!("Hello, world!"); 
} 
```

 说明：使用泛型的方式，则如例子中在实现trait的时候必须带上具体的类型，调用时也必须带上具体的类型。 

# 默认泛型类型参数和运算符重载

默认泛型类型参数和运算符重载 

（1）使用泛型类型参数时，可以为泛型指定一个默认的具体类型。 

（2）运算符重载是指在特定情况下自定义运算符行为的操作。Rust并不允许创建自定义运算符或者重载运算符，不过对于std::ops中列出的运算符和相应的trait，我们可以实现运算符相关trait来重载。 

（3）示例 

a、例子： 

```rust
use std::ops::Add; 
#[derive(Debug, PartialEq)] 
struct Point { 
  x: i32, 
  y: i32, 
} 

impl Add for Point { 
  type Output = Point; 
  fn add(self, other: Point) -> Point { 
    Point { 
      x: self.x + other.x, 
      y: self.y + other.y, 
    } 
  } 
} 

fn main() { 
  assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
} 
```

 b、Add trait 的定义如下： 

```rust
trait Add<RHS=Self> {  
  type Output;  
  fn add(self, rhs: RHS) -> Self::Output;  
}  
```

说明： 

尖括号中的 RHS=Self，这个语法叫做 默认类型参数（default type parameters）。RHS 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，上面的例子就是在默认类型上实现的 Add 的类型。 

自定义RHS的例子：  

```rust
use std::ops::Add;  
struct Millimeters(u32);  
struct Meters(u32);  
impl Add<Meters> for Millimeters {  
  type Output = Millimeters;  
  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000)) 
  } 
} 
```

 默认参数类型主要用于如下两个方面： 

- 扩展类型而不破坏现有代码。
- 在大部分用户都不需要的特定情况进行自定义。

# 完全限定语法

（1）同名的方法  

```rust
trait A{  
  fn print(&self); 
}  

trait B{  
  fn print(&self);  
}  

struct MyType;  

impl A for MyType{  
  fn print(&self) {  
    println!("A print for MyType"); 
  } 
}  

impl B for MyType{  
  fn print(&self) {  
    println!("B print for MyType");  
  } 
}  

impl MyType{  
  fn print(&self) {  
    println!("MyType"); 
  }  
} 

fn main() { 
  let my_type = MyType; 
  my_type.print(); //等价于MyType::print(&my_type); 
  A::print(&my_type); 
  B::print(&my_type); 
  println!("Hello, world!"); 
}  
```

说明：上述例子中，方法获取一个 self 参数，如果有两个 类型 都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现。（使用my_type.print()，print方法根据里面的self类型知道具体调用哪个方法） （2）对关联函数的完全限定语法 

例子： 

```rust
trait Animal {  
  fn baby_name() -> String; 
}  

struct Dog;  
impl Dog {  
  fn baby_name() -> String {  
    String::from("Spot")  
  }  
}  

impl Animal for Dog {  
  fn baby_name() -> String {  
    String::from("puppy") 
  }  
}  

fn main() {  
  println!("A baby dog is called a {}", Dog::baby_name()); 
  //println!("A baby dog is called a {}", Animal::baby_name());//报错，如何处理？  
} 
```

 正确的调用方式： 

```rust
fn main() {  
  println!("A baby dog is called a {}", Dog::baby_name()); 
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());//完全限定语法  
}
```

  完全限定语法定义为：

<Type as Trait>::function(receiver_if_method, next_arg, ...);

# 高级 trait

1、父 trait 用于在另一个 trait 中使用某 trait 的功能 

有时我们可能会需要某个 trait 使用另一个 trait 的功能。在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）。 

（1）错误例子：

```rust
use std::fmt; 
trait OutPrint: fmt::Display { 
  //要求实现DisPlay trait 
  fn out_print(&self) { 
    let output = self.to_string(); 
    println!("output: {}", output);
  }
} 

struct Point { 
  x: i32, 
  y: i32, 
} 

impl OutPrint for Point {} 

fn main() { println!("Hello, world!"); }  
```

（2）正确例子：  

```rust
use std::fmt; 
trait OutPrint: fmt::Display { 
  fn out_print(&self) { 
    let output = self.to_string(); 
    println!("output: {}", output); 
  } 
} 

struct Point { 
  x: i32, 
  y: i32, 
} 

impl OutPrint for Point {} 
impl fmt::Display for Point { 
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
    write!(f, "({}, {})", self.x, self.y) 
  } 
} 

fn main() { println!("Hello, world!"); } 
```

 2、newtype 模式用以在外部类型上实现外部 trait 

孤儿规则（orphan rule）：只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。一个绕开这个限制的方法是使用 newtype 模式（newtype pattern）。

 例子：

```rust
use std::fmt; 
struct Wrapper(Vec<String>); 
impl fmt::Display for Wrapper { 
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
    write!(f, "[{}]", self.0.join(", ")) 
  } 
} 

fn main() { 
  let w = Wrapper(vec![String::from("hello"), String::from("world")]); 
  println!("w = {}", w); 
} 
```

 说明：

 在上述例子中，我们在 Vec<T>上实现 Display，而孤儿规则阻止我们直接这么做，因为 Display trait 和 Vec<T>

 都定义于我们的 crate 之外。我们可以创建一个包含 Vec<T> 实例的 Wrapper 结构体，然后再实现。

