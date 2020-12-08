1、编写测试

```rust
 #[cfg(test)] 
mod tests { 
  #[test] 
  fn it_works() {
    assert_eq!(2 + 2, 4); 
  } 
}  
```

2、运行测试 

cargo test  

3、使用assert宏 

std::assert! 

std::assert_eq! 

std::assert_ne!

4.例子

animal.rs

```rust
pub mod dog {
    pub fn hello() {
        println!("wangwang");
    }

    pub fn is_dog() -> bool {
        true
    }
}

pub mod cat {
    pub fn hello() {
        println!("miaomiao");
    }

    pub fn is_cat() -> bool {
        true
    }
}
```

lib.rs

```rust
pub mod animal;

#[cfg(test)]
mod tests {
    //use animal::cat;
    //use animal::dog;
    use crate::animal::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn use_cat() {
        //cat::hello();
        assert_eq!(true, cat::is_cat());
    }

    #[test]
    fn use_dog() {
        //assert_eq!(true, animail::dog::is_dog());
        assert_eq!(true, dog::is_dog());
    }
}
```

