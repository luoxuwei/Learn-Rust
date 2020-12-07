1.rust中的方法类似于c++中类的成员函数。

2.方法例子

```rust
#[derive(Debug)]
struct Dog {
name: String,
weight: f32,
height: f32,
}
impl Dog { //下面为实现方法
fn get_name(&self) -> &str{
&(self.name[..])
}

fn get_weight(&self) -> f32 {
self.weight
}

fn get_height(&self) -> f32 {
self.height
}

}

fn main() {
let dog = Dog{name: String::from("jack"), weight:20.5, height: 60.0};
println!("name = {}", dog.get_name().to_string());
println!("weight = {}", dog.get_weight());
println!("height = {}", dog.get_height());
println!("Hello, world!");
}
```

3.可以有多个impl块 上面的代码

```rust
impl Dog { //下面为实现方法
fn get_name(&self) -> &str{
&(self.name[..])
}

fn get_weight(&self) -> f32 {
self.weight
}

fn get_height(&self) -> f32 {
self.height
}

}

//还可以写为多个impl块，如下：

impl Dog { //下面为实现方法
fn get_name(&self) -> &str{
&(self.name[..])
}
}

impl Dog { //下面为实现方法
fn get_weight(&self) -> f32 {
self.weight
}
}

impl Dog { //下面为实现方法
fn get_height(&self) -> f32 {
self.height
}
}
```

