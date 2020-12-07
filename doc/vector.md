```rust

//7、使用枚举
fn main() {
    //1.创建空的vector: Vec<T>
    //let v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    //v.push(1);

    //2.创建包含初始值的vector
    let v = vec![1, 2, 3];

    //3.丢弃vector
    {
        let v1 = vec![1, 2, 3];
      //此处离开v的作用域，v被丢弃，其中的元素也被丢弃
    }

    //4.读取元素
    //方法一：使用索引
    let one: &i32 = &v[0];
    //let four: &i32 = &v[3];
    println!("one = {}", one);
    println!("one = {}", *one);

    //方法二：使用get方法，rust推荐的方法
    //match v.get(1) {
    match v.get(3) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }
  
  //----复习知识点：不能在相同作用域下同时使用可变和不可变引用----
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0]; //不可变引用
  v.push(6);//改变
  println!("first = {}", first); //error，因为上一行已经变化（可变引用）


    //5.更新
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //6.遍历
    //(1)不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }

    //(2)可变的遍历
    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }


    //7.使用枚举存储多种类型
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };

    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001)
    ];



    //8、补充
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("first = {}", first);

    println!("Hello, world!");
}

```

