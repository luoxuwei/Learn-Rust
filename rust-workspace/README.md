## 讲解内容 
1、构建大型工程一定会涉及。  
2、例子1： （1）mkdir addr （2）cd add （3）创建Cargo.toml，加入如下内容：
[workspace]
 members = [ "adder", ]  
 （4）在当目录下运行cargo new adder 
 （5）运行cargo build构建工作空间 
 （6）可以用tree 来查看一下目录结构 
 （7）在Cargo.toml中添加内容，使之变成如下：
 [workspace] 
 members = [  "adder", “add-one”， ]  
 （8）运行cargo new add-one --lib 
 （9）在add-one/src/lib.rs中添加内容：  
 pub fn add_one(x: i32) -> i32 { x + 1 }  
 （10）在adder/Cargo.toml中添加
 [dependencies] 
 add-one = { path = "../add-one" }  
 （11）在adder/src/main.rs中添加：
 use add_one;
 fn main() { let num = 10; let r = add_one::add_one(num); println!("num = {}, r = {}", num, r); }  (12)cargo build构建工作空间
 （13）在顶层add目录运行二进制crate，需要通过-p参数和包名来运行：  cargo run -p adder  3、例子2：使用外部包 参考之前使用过rust-crypto

