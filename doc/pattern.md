# 模式

1、模式是Rust中特殊的语法，模式用来匹配值的结构。 

2、模式由如下内容组成： 

（1）字面值 

（2）解构的数组、枚举、结构体或者元组 

（3）变量 

（4）通配符 

（5）占位符 

3、在Rust中，可能用到模式的位置 

（1）match分支 

匹配方式： 

 match VALUE { 

PATTERN => EXPRESSION, 

PATTERN => EXPRESSION, 

PATTERN => EXPRESSION, 

}  

要点：所有情况要匹配完全（即，必须穷尽所有情况），否则报错

 例子： 

```rust
 fn main()
{ 
  let a = 1; 
  match a { 
    0 => println!("zero"), 
    1 => println!("one"),
    _ => println!("other"),//这一句如果注释掉会报错 
  };
}  
```

（2）if let条件表达式 

有选择的匹配，if let 表达式其穷尽性没有为编译器所检查。

 例子： 

```rust
 fn main() {
   let color: Option<&str> = None; 
   let is_ok = true; 
   let age: Result<u8,_> = "33".parse(); 
   if let Some(c) = color {
     println!("color = {}", c); 
   } else if is_ok { 
     println!("is ok? {}", is_ok); 
   } else if let Ok(a) = age { 
     if a > 30 { 
       println!("oh, mature man"); 
     } else { 
       println!("oh, young man"); 
     } 
   } else { 
     println!("in else"); 
   } 
}  
```

（3）while let 

允许只要模式匹配就一直进行 while 循环。 

例子：  

```rust
fn main() { 
  let mut stack = Vec::new();  
  stack.push(1);  
  stack.push(2);  
  stack.push(3);  
  while let Some(top) = stack.pop() { 
    println!("{}", top);  
  } 
}  
```

在上述例子中，一直匹配Some(value)，只要匹配就一直循环。 

（4）for循环 

在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。 

例子：  

```rust
fn main() { 
  let v = vec!['a', 'b', 'c'];  
  for (index, value) in v.iter().enumerate() {  
    println!("{} is at index {}", value, index);  
  } 
}  
```

说明：这里的模式为（index， value），enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，它们位于一个元组中。第一个 enumerate 调用会产生元组 (0, 'a')。当这个值匹配模式 (index, value)，index 将会是 0 而 value 将会是 'a'，并打印出第一行输出。 

（5）let语句

  let PATTERN = EXPRESSION; 

 例子： 

 let （x， y， z） = （1，2，3）；  

说明：（1，2，3）匹配模式（x，y，z），会把 1 绑定到 x，2 绑定到 y 并将 3 绑定到 z。 如果希望忽略元组中一个或多个值，也可以使用 _ 或 .. 

例子：  

let (x, .., y) = (1, 2, 3); 

println!("x: {}, y: {}", x, y);  

（6）函数参数 

函数的参数也是模式。 

例子： 

```rust
fn print_point(&(x, y): &(i32, i32)) {  
  println!("p: ({}, {})", x, y); 
}  
fn main() {  
  let point = (3, 5);  
  print_point(&point);  
}  
```

说明：值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5。 

4、模式在每个使用它的地方并不以相同的方式工作；在一些地方，模式必须是 irrefutable （不可反驳）的，意味着他们必须匹配所提供的任何值。在另一些情况，则是 refutable 的。