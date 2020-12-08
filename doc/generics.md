1、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复。  

2、在函数定义中使用泛型。 例子： 

++++++++不使用泛型+++++++++++ 

```rust
 //for i32 
fn largest_i32(list: &[i32]) -> i32 { 
  let mut leargest = list[0]; 
  for &item in list.iter() { 
    if item > largest { 
      largest = item; 
    } 
  } 
  largest 
}  
//for char 
fn largest_char(list: &[char]) -> char { 
  let mut leargest = list[0]; 
  for &item in list.iter() { 
    if item > largest { 
      largest = item; 
    } 
  } 
  largest 
}  

fn main(){ 
  let number_list = vec![1, 2, 22, 3, 42]; 
  let r1 = largest_i32(&number_list); 
  println!("r1 = {}", r1); 
  let char_list = vec!['a', 'y', 'c', 'd']; 
  let r2 = largest_char(&char_list); 
  println!("r2 = {}", r2); 
}  
```

```rust
//+++++++++使用泛型：会报错的函数++++++++++++ 
fn largest<T：PartialOrd + Copy>(list: &[T]) -> T { //注意，要实现比较和复制的trait才行，否则报错
let mut largest = list[0];
for &item in list.iter() {
if item > largest {
largest = item;
}
}
largest
}
fn main(){
let number_list = vec![1, 2, 22, 3, 42];
let r1 = largest_i32(&number_list);
println!("r1 = {}", r1);
let char_list = vec!['a', 'y', 'c', 'd'];
let r2 = largest_char(&char_list);
println!("r2 = {}", r2);
}
```

 3、在结构体中使用泛型。

例子1： 

```rust
#[derive(Debug)]
struct Point<T> {
x: T,
y: T,
}
fn main() {
let integer = Point {x: 1, y: 2};
println!("{:#?}", integer);
let float = Point {x: 0.99, y: 1.99};
println!("{:#?}", float);
}
```

 例子2：  

```rust
#[derive(Debug)]
struct Point<T, U> {
x: T,
y: U,
}

fn main() {
let a = Point {x: 1, y: 2.0};
println!("{:#?}", a);
let b = Point {x: 1, y: 1.99};
println!("{:#?}", b);
}
```

  4、枚举中的泛型  

```rust
//复习
enum Option<T> {
Some(T),
None,
}

enum Result<T, E> {
Ok(T),
Err(e),
}
```

  5、方法中的泛型  

```rust
struct Point<T> {
x: T,
y: T,
}

impl<T> Point<T> {
fn get_x(&self) -> &T {
&self.x
}

fn get_y(&self) -> &T {
&self.y
}
}
fn main() {
let p = Point {x: 1, y: 2};
println!("p.x = {}", p.get_x());
println!("p.y = {}", p.get_y());
}
```

  例子2：方法和结构体中使用不同的类型  

```rust
struct Point<T, U> {
x: T,
y: U,
}

impl<T, U> Point<T, U> {
fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
Point {
x: self.x,
y: other.y,
}
}
}

fn main() {
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c'};
let p3 = p1.mixup(p2);
println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

  6、使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。