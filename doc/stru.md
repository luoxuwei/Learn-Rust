1.定义结构体

```rust
struct User {
username: String,
email: String,
sign_in_count: u64,
active: bool,
}
```

2.创建结构体实例

```rust
let user1 = User {
email: String::from("someone@example.com"),
username: String::from("someusername123"),
active: true,
sign_in_count: 1,
};
```

3.修改结构体字段

```rust
let mut user1 = User {
email: String::from("someone@example.com"),
username: String::from("someusername123"),
active: true,
sign_in_count: 1,
};
user1.sign_in_count = 3；
```

4.参数名字和字段名字同名的简写方法

```rust
fn build_user(email: String, username: String) -> User {
User {
email,
username,
active: true,
sign_in_count: 1,
}
}
```

5.从其它结构体创建实例

```rust
let user2 = User {
email: String::from("another@example.com"),
username: String::from("anotherusername567"),
active: user1.active,
sign_in_count: user1.sign_in_count,
};
//简写方法：
let user2 = User {
email: String::from("another@example.com"),
username: String::from("anotherusername567"),
..user1
};
```

6.元组结构体 特点： 

（1）字段没有名字 

（2）圆括号

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

7.没有任何字段的类单元结构体

用于在其上实现trait，类似于定义一个没有成员的类，但是有成员函数。

8.打印结构体

```rust
#[derive(Debug)]       //添加derive(Debug)支持打印
struct Rectangle {
width: u32,
height: u32,
}
fn main() {
let rect1 = Rectangle { width: 30, height: 50 };
println!("rect1 is {:?}", rect1);
}
```

