```rust
//1、类似于c语言的方式定义
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
kind: IpAddrKind::V4,
address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
kind: IpAddrKind::V6,
address: String::from("::1"),
};

//2、rust语言提倡的方式定义
enum IpAddr2 {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

//3、可以是不同类型
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

//4、经典用法
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}

/*解释：
* Quit 没有关联任何数据。
* Move 包含一个匿名结构体。
* Write 包含单独一个 String。
* ChangeColor 包含三个 i32。*/


//等同于
//struct QuitMessage; //类单元结构体
//struct MoveMessage {
//  x: i32,
//  y: i32,
//}
//struct WriteMessage(String) 
//struct Change(i32, i32, i32)

//5、枚举类型的方法以及match
impl Message {
    fn prin(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            _ => println!("Write")
            //Message::Write(&s) => println!("Write = {}", s)
        }
    }
}

fn main() {
    let i1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let i2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));

    let quit = Message::Quit;
    quit.prin();


    let mo = Message::Move{x: 10, y: 20};
    mo.prin();

    let wri = Message::Write(String::from("Hello"));
    wri.prin();

    let change = Message::Change(1, 2, 3);
    change.prin();
    println!("Hello, world!");
}

```

