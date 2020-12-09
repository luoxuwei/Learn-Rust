# trait 对象的例子

trait对象的例子：

 (1) mkdir learn_oo2 

(2)cd learn_oo2，编辑Cargo.toml 

 [workspace]

 members = [    

 "gui",     

"main",

 ] 

 (3) cargo new gui --lib

 (4) 编辑gui/src/lib.rs源码  

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //trait对象，使用dyn关键字
}

//void func(A *p){
//}
//
//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

//impl<T> Screen <T>
//    where T: Draw {
//    pub fn run(&self) {
//        for comp in self.components.iter() {
//            comp.draw();
//        }
//    }
//}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}", 
                 self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, option = {:?}", 
                 self.width, self.height, self.option);
    }
}

////复习
//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}
//
//impl<T> Screen<T>
//    where T: Draw {
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

 (5) cargo new main

 (6) 编辑Cargo.toml文件，添加：

 [dependencies] 

gui = {path = "../gui"}

 (7) 编辑src/main.rs源码 

```rust
use gui::{Screen, Button, SelectBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };

    //let s = Screen {
    //    components: vec![
    //        Button {
    //            width: 50,
    //            height: 10,
    //            label: String::from("ok"),
    //        },
    //        SelectBox {
    //            width: 60,
    //            height: 40,
    //            option: vec![
    //                String::from("Yes"),
    //                String::from("No"),
    //                String::from("MayBe"),
    //            ],
    //        },
    //    ],
    //};

    s.run();
    println!("Hello, world!");
}
```

# trait 对象

1、trait对象动态分发 

（1）在上一节的trait对象例子中，对泛型类型使用trait bound编译器进行的方式是单态化处理，单态化的代码进行的是静态分发（就是说编译器在编译的时候就知道调用了什么方法）。 

（2）使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。

 2、trait对象要求对象安全

 只有 对象安全（object safe）的 trait 才可以组成 trait 对象。trait的方法满足以下两条要求才是对象安全的：

返回值类型不为 Self 

方法没有任何泛型类型参数 

例子： 

```rust
//错误，因为Clone是非对象安全的，所以不能作为trait对象
//pub trait Clone {
//    fn clone(&self) -> Self;
//}

//pub struct Screen {
//    pub com: Vec<Box<dyn Clone>>,
//}
fn main() {  println!("Hello, world!");}
```



