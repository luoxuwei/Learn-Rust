# 迭代器介绍

1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。  2、创建一个迭代器：迭代器是惰性的，意思是在调用方法使用迭代器之前，它都不会有任何效果。 

例子：  

let v1 = vec![1, 2, 3]; 

let v1_iter = v1.iter();  

 3、使用迭代器 

例子：在for循环中使用迭代器 

 let v1 = vec![1, 2, 3];

 let v1_iter = v1.iter();

 for val in v1_iter { 

println!("Got: {}", val);

 }   

4、Iterator trait：迭代器都实现了Iterator trait，这个trait定义于标准库中。该trait定义如下：

  trait Iterator {

 type Item; 

fn next(mut self) -> Option; //省略其它内容 }  

说明：type Item和Self::Item这种用法叫做定义trait的关联类型。 

 Item类型将是迭代器返回元素的类型，next方法是Iterator实现者被要求定义的唯一方法，next一次返回一个元素，当迭代器结束，则返回None。  

5、next方法：迭代器通过next方法来消费一个项。 

（1）不可变引用的使用迭代器 

```rust
let v1 = vec![1, 2, 3]; 
let mut v1_iter = v1.iter(); 
if let Some(v) = v1_iter.next() { 
  println!("{}", v); //1
} 

if let Some(v) = v1_iter.next() { 
  println!("{}", v); //2 
}  

if let Some(v) = v1_iter.next() {
  println!("{}", v); //3
}  

if let Some(v) = v1_iter.next() { 
  println!("{}", v); //3 
} else { 
  println!("Reached end!") //printf "Reached end" 
}   
```

（2）如果希望迭代可变引用，则可以调用iter_mut。 

例子： 

```rust
let mut v2 = vec![1, 2, 3]; 
let mut v2_iter = v2.iter_mut();
if let Some(v) = v2_iter.next() {
  *v = 3; 
} 
let it = v2.iter();  
//打印：3，2，3 
for val in it { 
  println!("{}", val);
}
```

# 适配器

１、消费适配器 

Iterator trait有一系列由标准库提供的默认实现的方法，有些方法调用了next方法，这些调用next方法的方法被称为消费适配器。 

例子如下：  

```rust
let v1 = vec![1, 2, 3]; 
let v1_iter = v1.iter(); 
let total: i32 = v1_iter.sum();//调用消费适配器sum来求和 
```

 ２、迭代器适配器 

Iterator trait中定义了一类方法，被称为迭代器适配器，即允许我们将当前迭代器变为不同类型的迭代器。

 例子： 

```rust
let v1: Vec = vec![1, 2, 3]; 
//v1.iter().map(|x| x + 1); //创建了一个新的迭代器，但是由于迭代器的惰性，什么也不会发生 
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //必须要消费迭代器才会生效，v2 = vec![2, 3, 4] 
```

 例子： 

```rust
 fn main() { 
   let v1: Vec<i32> = vec![1, 11, 5, 34, 2, 10]; 
   let v2: Vec<i32> = v1.into_iter().filter(|x| *x > 5).collect(); 
   println!("v2 = {:?}", v2); 
}
```

# 自定义迭代器

１、实现Iterator trait来创建自定义迭代器 

例子：  

```rust
struct Counter {
  count: u32, 
} 

impl Counter { 
  fn new() -> Counter { 
    Counter { count: 0 }
  } 
} 
//实现自定义迭代器 
impl Iterator for Counter { 
  type Item = u32; 
  fn next(&mut self) -> Option<Self::Item>{ 
    self.count += 1; 
    if self.count < 6 { 
      Some(self.count) 
    } else { 
      None 
    } 
  } 
} 
//使用 
fn main() { 
  let mut counter = Counter::new(); 
  for i in (0..6) { 
    if let Some(v) = counter.next() { 
      println!("i = {}, v = {}", i, v); 
    } else { 
      println!("i = {}, at end", i); 
      break; 
    } 
  } 
} 
```

 2、迭代器并不会引入其它的性能开销









