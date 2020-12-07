1.例子

```rust
fn main() {
let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
let s2 = String::from("hello"); // s2 进入作用域
let s3 = takes_and_gives_back(s2); // s2 被移动到takes_and_gives_back 中, 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 移出作用域并被丢弃
fn gives_ownership() -> String { // gives_ownership 将返回值移动给调用它的函数
let some_string = String::from("hello"); // some_string 进入作用域.
some_string // 返回 some_string 并移出给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
a_string // 返回 a_string 并移出给调用的函数
}
```

移动来移动去的很麻烦，可能我只是想使用，怎么办？

2.引用

例子：

```rust
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生
{
let s1 = String::from("hello");
let len = calculate_length(&s1);
}
```

&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。

3.借用

要修改的时候，就需要借用

```rust
fn main() {
let mut s = String::from("hello");
change(&mut s);
}

fn change(some_string: &mut String) {
some_string.push_str(", world");
}
```

值得注意的是：也不能在拥有不可变引用的同时拥有可变引用。

例子：

```rust
let mut s = String::from("hello");
let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题
println!("{}, {}, and {}", r1, r2, r3); //r2、r1不希望在使用时被改变，因此此处error
```

以下是ok的，例子：

```rust
let mut s = String::from("hello");
let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{} and {}", r1, r2);
// 此位置之后 r1 和 r2 不再使用
let r3 = &mut s; // 没问题

```

4.悬垂引用error

```rust
fn main() {
let reference_to_nothing = dangle();
}

fn dangle() -> &String {
let s = String::from("hello");
&s //error，正确的方式是返回s
}
```

5.总结： 

（1）在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。 

（2）引用必须总是有效。