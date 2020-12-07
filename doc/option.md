```rust
//1、Option是标准库定义的一个枚举，形式：
//enum Option<T> {
//  Some(T),
//  None,
//}
//
//2、使用方式
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y {
        Some(i) => { temp = i; }
        None => {println!("do nothing");}
    }

    let sum = x + temp;
    println!("sum = {}", sum);
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; //error，因为类型不一样

    //let result = plus_one(y);
    //match result {
    //    Some(i) => println!("result = {}", i),
    //    None => println!("nothing"),
    //};

    if let Some(value) = plus_one(y) {
        println!("value = {}", value);
    }

    
    if let Some(value) = plus_one(y) {
        println!("value = {}", value);
    } else {
        println!("do nothing");
    }

    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x+1),
    }
}
```
3、使用Option主要是用来对空值进行处理的，在rust里面就必须将空值放入Option，因此能减少bug

4、匹配
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
match x {
None => None,
Some(i) => Some(i + 1),
}
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

以下代码错误，因为匹配没有处理完：
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
match x {
Some(i) => Some(i + 1),
}
}
```

知识点拾遗——Option 将其中的引用转换为值

```rust
fn main() {                                                                                        
      let x = 123u8;         
      let y: Option<&u8> = Some(&x);  
      let z: Option<u8> = y.copied(); 

      if let Some(xx) = z {
          println!("xx == {}", xx);
      }
      println!("Hello, world!");
 }

//在上述代码中，通过y.copied将引用转换为值，这在实际项目中非常实用。
//copied为option的一个方法，想详细研究option方法的可以参考文档https://doc.rust-//lang.org/std/option/enum.Option.html
```

