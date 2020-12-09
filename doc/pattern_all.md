# 字面/命名变量/多个模式/..=匹配

```rust
//1、匹配字面值
//fn main() {
//    let x = 1;
//    match x {
//        1 => println!("one"),
//        2 => println!("two"),
//        _ => println!("xx"),
//    };
//
//    println!("Hello, world!");
//}

////2、匹配命名变量
//fn main() {
//    let x = Some(5);
//    let y = 10; //位置1
//    match x {
//        Some(50) => println!("50"),
//        Some(y) => println!("value = {}", y), //此处的y和上面1处的y不一样，此处是引入的变量y覆盖之前的y
//        _ => println!("other"),
//    };
//
//    println!("x = {:?}, y = {:?}", x, y); //此处的y是位置1的y
//}

////3、多个模式
//fn main() {
//    let x = 1;
//    match x {
//        1|2 => println!("1 or 2"), //|表示是或，匹配1或者2
//        3 => println!("3"),
//        _ => println!("xx"),
//    };
//}

//4、通过..匹配
fn main() {
    //let x = 5;

    //match x {
    //    1..=5 => println!("1 to 5"), // 1|2|3|4|5 => println!("1 to 5"),
    //    _ => println!("xx"),
    //};
//说明：在例子中， 1..=5 等价于 1 | 2 | 3 | 4 | 5

    let x = 'c';
    match x {
        'a'..='j' => println!("1"),
        'k'..='z' => println!("2"),
        _ => println!("other"),
    }
}
```

# 解构并分解值

可以使用模式来解构结构体、枚举、元组和引用，以便使用这些值的不同部分。 

（1）解构结构体

 例子： 

```rust
struct Point {   
  x: i32,   
  y: i32, 
} 

fn main() {   
  let p = Point { x: 0, y: 7 };   
  let Point { x: a, y: b } = p;   
  assert_eq!(0, a);   
  assert_eq!(7, b);   
  //也可以写成：   
  //let Point { x, y } = p;  //创建了同名的变量，可以简写   
  //assert_eq!(0, x);   
  //assert_eq!(7, y); 
} 
```

 说明：变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段。 

也可以使用字面值作为结构体模式的一部分进行进行解构，而不是为所有的字段创建变量。例子如下：  

```rust
fn main() {   
  let p = Point { x: 0, y: 7 };   
  match p {     
    Point { x, y: 0 } => println!("On the x axis at {}", x),     
    Point { x: 0, y } => println!("On the y axis at {}", y),     
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),   
  } 
}  
```

（2）解构枚举类型 

例子（复习）：  

```rust
enum Message {   
  Quit,   
  Move { x: i32, y: i32 },   
  Write(String),   
  ChangeColor(i32, i32, i32), 
} 

fn main() {   
  let msg = Message::ChangeColor(0, 160, 255);   
  match msg {     
    Message::Quit => {       
      println!("The Quit variant has no data to destructure.")     
    }     
    Message::Move { x, y } => {       
      println!(         
        "Move in the x direction {} and in the y direction {}",         
        x,         
        y       
      );     
    }     
    Message::Write(text) => println!("Text message: {}", text),     
    Message::ChangeColor(r, g, b) => {       
      println!(         
        "Change the color to red {}, green {}, and blue {}",         
        r,         
        g,         
        b       
      )     
    }   
  } 
} 
```

 说明： 对于 Message::Quit 这样没有任何数据的枚举成员，不能进一步解构其值。只能匹配其字面值 Message::Quit；

 对于像 Message::Move 这样的类结构体枚举成员，可以采用类似于匹配结构体的模式；

 对于像 Message::Write 这样的包含一个元素，以及像 Message::ChangeColor 这样包含两个元素的类元组枚举成员，其模式则类似于用于解构元组的模式。 

（3）解构嵌套的结构体和枚举 

例子：  

```rust
enum Color {   
  Rgb(i32, i32, i32),   
  Hsv(i32, i32, i32), 
} 

enum Message {   
  Quit,   
  Move { x: i32, y: i32 },   
  Write(String),   
  ChangeColor(Color), 
} 

fn main() {   
  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));   
  match msg {     
    Message::ChangeColor(Color::Rgb(r, g, b)) => {       
      println!(         
        "Change the color to red {}, green {}, and blue {}",         
        r,         
        g,         
        b       
      )     
    }     
    Message::ChangeColor(Color::Hsv(h, s, v)) => {       
      println!(         
        "Change the color to hue {}, saturation {}, and value {}",         
        h,         
        s,         
        v       
      )     
    }     
    _ => ()   
  } 
}  
```

（4）解构结构体和元组 

例子： 

```rust
struct Point{   
  x: i32,   
  y: i32, 
} 

fn main() {   
  let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });   
  println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);   
  println!("Hello, world!"); 
}
```

# 忽略模式中的值

1、忽略模式中的值 

（1）使用\_忽略整个值 

例子：  

```rust
fn foo(_: i32, y: i32) {   
  println!("This code only uses the y parameter: {}", y); 
} 

fn main() {   
  foo(3, 4); 
} 
```

 说明：对函数中的参数使用占位符，主要是在实现trait的时候使用（如不需要特征指定函数中的某个参数）。 （2）使用\_忽略部分值 

```rust
let numbers = (2, 4, 8, 16, 32); 
match numbers {   
  (first, _, third, _, fifth) => {     
    println!("Some numbers: {}, {}, {}", first, third, fifth)   
  }, 
}  
```

（3）通过在名字前以一个下划线开头来忽略未使用的变量 

例子1： 

```rust
fn main() {   
  let _x = 5;   
  let y = 10; 
} 
```


说明：上述代码中，编译时对未使用y警告，对未使用x不会警告。 

只使用 _ 和使用以下划线开头的名称有些微妙的不同：比如\_x 仍会将值绑定到变量，而 \_ 则完全不会绑定。

 例子2：  

```rust
//会报错，因为以下划线开头的变量仍然会绑定值 
let s = Some(String::from("Hello!")); 
if let Some(_s) = s {   
  println!("found a string"); 
} 
println!("{:?}", s); 
```

 例子3：  

```rust
//不会报错，因为s的所有权不会移动到\_中 
let s = Some(String::from("Hello!")); 
if let Some(_) = s {   
  println!("found a string"); 
} 
println!("{:?}", s);  
```

（4）用..忽略剩余值 

对于有多个部分的值，可以使用 .. 语法来只使用部分并忽略其它值 

例子1：  

```rust
fn main() {   
  let numbers = (2, 4, 8, 16, 32);   
  match numbers {     
    (first, .., last) => { //自动匹配到第一个和最后一个值       
      println!("Some numbers: {}, {}", first, last);     
    },   
  } 
} 
```

 例子2： 

```rust
 //下面的例子错误，因为..匹配必须是无歧义的 
fn main() {   
  let numbers = (2, 4, 8, 16, 32);   
  match numbers {     
    (.., second, ..) => {       
      println!("Some numbers: {}", second)     
    },   
  } 
}
```

# 匹配守卫和绑定

1、匹配守卫提供的额外的条件 

匹配守卫是一个指定于match分支模式之后的额外的if条件，它必须满足才能选择此分支。

 例子1：  

```rust
let num = Some(4); 
match num {   
  Some(x) if x < 5 => println!("less than five: {}", x),  //匹配守卫   
  Some(x) => println!("{}", x),   
  None => (), 
}  
```

例子2：  

```rust
fn main() {   
  let x = Some(5);   
  let y = 10;   //位置1   
  match x {     
    Some(50) => println!("Got 50"),     
    Some(n) if n == y => println!("Matched, n = {}", n),  //此处的y就是位置1处的y，不是额外创建的变量     
    _ => println!("Default case, x = {:?}", x),   
  }   
  println!("at the end: x = {:?}, y = {}", x, y); 
}
```

  例子3： 

```rust
let x = 4; 
let y = false; 
match x {   
  4 | 5 | 6 if y => println!("yes"), //等价于（4 | 5 | 6） 
  if y => println!("yes"),   
  _ => println!("no"), 
}  
```

2、绑定 

@运算符允许我们在创建一个存放值的变量，并且测试这个变量的值是否匹配模式。 

例子：  

```rust
enum Message {   
  Hello { id: i32 }, 
} 

let msg = Message::Hello { id: 5 }; 
match msg {   
  Message::Hello { id: id_variable @ 3..=7 } => {  //创建id_variable 存放id的值，同时测试值是否在3到7的范围     
    println!("Found an id in range: {}", id_variable)   
  },   
  Message::Hello { id: 10..=12 } => {     
    println!("Found an id in another range")   
  },   
  Message::Hello { id } => {     
    println!("Found some other id: {}", id)   
  }, 
}
```









