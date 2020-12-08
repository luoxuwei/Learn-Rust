# Drop trait 介绍

1、Drop trait类似于其它语言中的析构函数，当值离开作用域时执行此函数的代码。

 可以为任何类型提供Drop trait的实现（但是注意，这里的类型需要用struct包含起来，用例子实现Drop for i32和Drop for String报错）。 

2、实现Drop trait 

例子： 

```rust
struct Dog { 
  name: String, 
} //下面为Dog实现Drop trait 
impl Drop for Dog { 
  fn drop( &mut self ) { 
    println!("Dog leave"); 
  } 
} 
fn main() { 
  let a = Dog { 
    name: String::from("wangcai")
  }; 
  let b = Dog { 
    name: String::from("dahuang")
  }; 
  println!("At the end of main"); 
} //从打印结果可以看出是离开作用域时调用了drop方法。
```

# 提前调用 drop

通过std::mem::drop提早丢弃值。当需要提前进行清理时，不是直接调用drop方法，而是调用是std::mem::drop方法，例如如下： 

```rust
struct Dog { 
  name: String, 
} 
//下面为Dog实现Drop trait 
impl Drop for Dog { 
  fn drop( &mut self ) {
    println!("Dog leave"); 
  } 
} 

fn main() { 
  let a = Dog { 
    name: String::from("wangcai")
  }; 
  let b = Dog { 
    name: String::from("dahuang")
  }; 
  //a.drop();
  //错误，不能直接调用drop 
  drop(a);
  //正确，通过std::mem::drop显示清理 
  println!("At the end of main"); 
}  
```

上述例子打印的顺序将是：  

Dog leave //调用drop(a)的打印 

At the end of main 

Dog leave

