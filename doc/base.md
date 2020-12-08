## 

- Rust变量

```rust
const MAX_POINTS: u32 = 100000;
fn main() {
    //1、变量定义
    //定义变量用let，如果变量没有用mut，那么是不可变的
    //let name: type 
    let a = 1;
    //let a:u32 = 1;
    println!("a = {}", a);

    let mut b: u32 = 1;
    println!("b = {}", b);

    b = 2;
    println!("b = {}", b);

    //2、隐藏
    let b: f32 = 1.1;
    println!("b = {}", b);

    //3、常量
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("Hello, world!");
}
```

- Rust类型

```rust
fn main() {
    //bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);

    let is_false = false;
    println!("is_false = {}, {}", is_false, is_true);

    //char 在rust里面，char是32位的
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);
    
    //i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let c: i8 = -111;
    println!("c = {}", c);

    let d: f32 = 0.0009;
    println!("d = {}", d);

    //自适应类型isize, usize
    println!("max = {}", usize::max_value());

    //数组
    //[Type; size] , size 也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let arr1: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    show(arr1);

    //元组
    let tup: (i32, f32, char) = (-3, 3.69, '好');
    let tup = (-3, 3.69, '好');
    println!("--------------------");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("--------------------");
    
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("Hello, world!");
}

fn show(arr:[u32;3]) {
    println!("--------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("--------------------");
}
```

- Rust注释
- Rust控制流

```rust
fn main() {
    //if 
    let y = 0;
    if y == 1 {
        println!("y = 1");
    }

    //if-else
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }

    //if - else if - else
    println!("++++++++++++");
    let y = 2;
    if y == 1 {
        println!("y = 1");
    } else if y == 0 {
        println!("y = 0");
    } else if y == 2 {
        println!("y = 2");
    } else {
        println!("other");
    }

    //let中使用if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
        //"six" //error
    };
    
    println!("x = {}", x);

    //loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }

        //counter = counter + 1;
        counter += 1;
    }
    
    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter*2;
        }
    };
    println!("result = {}", result);

    //while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    //for
    println!("+++++++++++++++");
    let arr:[u32; 5] = [1, 2, 3, 4, 5];
    //for element in arr.iter() {
    for element in &arr {
        println!("element = {}", element);
    }
    println!("Hello, world!");
}
```

- Rust函数

```rust
fn other_fun() {
    println!("This is a function");
}

fn other_fun1(a: i32, b: u32) {
//fn other_fun1(a, b) { //error
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn other_fun3(a: i32, b: i32) -> i32 {
    //let result = a + b;
    //result
    a + b
}

fn main() {
    other_fun();

    let a: i32 = -1;
    let b: u32 = 2;
    other_fun1(a, b);

    let c: i32 = 9;
    let r: i32 = other_fun2(a, c);
    println!("r = {}", r);

    let r2: i32 = other_fun3(a, c);
    println!("r2 = {}", r2);

    //语句是执行一些操作，但是不返回值的指令
    //let y = 1; //语句，不返回值
    //let x = (let y = 1);
    
    //表达式会计算一些值
    let y = {
        let x = 1;
        //x + 1;
        x + 1
    };

    println!("y = {}", y);

    println!("Hello, world!");
}
```

## 