1.字符串slice 字符串 slice（string slice）是 String 中一部分值的引用。例子：

```rust
let s = String::from("hello world");
let hello = &s[0..5]; //包含下标0，不包含5
let world = &s[6..11];//包含下标6，不包含下标11
let hello = &s[0..=4];//包含下标4
let world = &s[6..=10];//包含下标10
```

也可以写为：

```rust
let s1 = &s[2..];
let s2 = &s[..3];
```

注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集。

2.字面值就是slice

字符串字面值被储存在二进制文件中吗。

```rust
let s = "Hello, world!";
```

这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

3.其它类型的slice

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

