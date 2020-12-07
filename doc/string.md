在rust核心语言中，只有一种字符串类型，就是str，意思就是string slice。通常，我们在rust语言中谈论string的时候通常包括str和标准库中的String。str和String都是UTF-8编码的。

```rust
//https://www.bilibili.com/read/cv4251500
fn main() {
    //1、创建一个空String
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
  
    //2、通过字面值创建一个String
    //2.1、使用String::from()
    let s1 = String::from("init some thing");
    println!("{}", s1);
    //2.2、使用str的方式
    let s1 = "init some thing".to_string();
    println!("{}", s1);
  
    //3、更新String
    //3.1、push_str:这种方式能添加一个str到String
    let mut s3 = String::from("update string");
    println!("before push, s3 = {}", s3);
    s3.push_str(", push some thing");
    println!("after push, s3 = {}", s3);
    let s31 = ", push another";
    s3.push_str(s31);
    println!("after second push, s3 = {}", s3);
    //push_str 并不会获取s31的所有权，接下来能继续使用

    //3.2、使用push：只能添加一个字符
    let mut s2 = String::from("tea");
    s2.push('m');
    //s2.push('mx'); //error
    //s2.push("x");  //error
    println!("{}", s2);

    //3.3、使用“+”合并字符串
    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    //3.4、使用format!
    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}", s341, s342, s343); //format!和println!类似
    println!("s344 = {}", s344);
    println!("s341 = {}", s341);
    println!("s342 = {}", s342);
    println!("s343 = {}", s343);

    //4 索引String: rust 不支持字符串索引
    let s4 = String::from("hello");
    //let h = s4[0]; //错误
    let s41 = "hello";
    //let h2 = s41[0];//错误
    //原因
    let len1 = String::from("hello").len();
    println!("len1 = {}", len1);
    let len2 = String::from("你好").len();
    println!("len2 = {}", len2); //期望的是 2 ?, 实际上 6, 一个UFT-8字符并不一定是一个确定的Unicode字符, 因此不能找到确定的索引


    //5、str 索引
    let hello = "你好";
    let h5 = &hello[0..3];
    println!("h5 = {}", h5); 
    let s51 = "hello";
    let h51 = &s51[0..1];
    println!("h51 = {}", h51);


    //6、遍历
    //6.1、chars
    //chars
    for c in s4.chars() {
        println!("c = {}", c);
    }

    println!("+++++++++++++++");
    //6.2、bytes
    //bytes
    for b in s4.bytes() {
        println!("b = {}", b);
    }
    println!("+++++++++++++++");
    println!("Hello, world!");
}
```

