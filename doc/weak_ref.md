```rust
#[derive(Debug)]
enum List {
    //Cons(i32, RefCell<Rc<List>>),
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::rc::Weak;

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("2, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());


    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("3, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", a.tail());

    println!("Hello, world!");
}

//弱引用Weak<T>
//特点：
//（1）弱引用通过Rc::downgrade传递Rc实例的引用，调用Rc::downgrade会得到Weak<T>类型的智能指针，同时将weak_count加1（不是将strong_count加1）。
//（2）区别在于 weak_count 无需计数为 0 就能使 Rc 实例被清理。只要strong_count为0就可以了。
//（3）可以通过Rc::upgrade方法返回Option<Rc<T>>对象。
```

用Weak<>创建树形数据结构 

```rust
//（1）定义结构体  
use std::rc::{Rc, Weak};  
use std::cell::RefCell;  
#[derive(Debug)]  
struct Node {  
  value: i32,  
  parent: RefCell<Weak<Node>>,  
  children: RefCell<Vec<Rc<Node>>, 
}  
//（2）使用示例  
fn main() {  
  let leaf = Rc::new(Node {  
    value: 3,  
    parent: RefCell::new(Weak::new()),  
    children: RefCell::new(vec![]),  
  });  
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  
  let branch = Rc::new(Node {  
    value: 5,  
    parent: RefCell::new(Weak::new()),  
    children: RefCell::new(vec![Rc::clone(&leaf)]),  
  });   
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);  
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

通过例子查看strong_count和weak_count的改变 

例子：  

```rust
use std::rc::{Rc, Weak}; 
use std::cell::RefCell; 
#[derive(Debug)] 
struct Node {   
  value: i32,   
  parent: RefCell<Weak<Node>>,   
  children: RefCell<Vec<Rc<Node>>, 
} 

fn main() {   
  let leaf = Rc::new(Node {     
    value: 3,     
    parent: RefCell::new(Weak::new()),     
    children: RefCell::new(vec![]),   
  });   
  println!(
    "leaf strong = {}, weak = {}", 
    Rc::strong_count(&leaf), 
    Rc::weak_count(&leaf),
  );   
  {     
    let branch = Rc::new(Node {       
      value: 5,       
      parent: RefCell::new(Weak::new()),       
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });     
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);     
    println!(
      "branch strong = {}, weak = {}", 
      Rc::strong_count(&branch),       
      Rc::weak_count(&branch),     
    );     
    println!(       
      "leaf strong = {}, weak = {}",       
      Rc::strong_count(&leaf),       
      Rc::weak_count(&leaf),     
    );   
  }   
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());   
  println!(     
    "leaf strong = {}, weak = {}",     
    Rc::strong_count(&leaf),     
    Rc::weak_count(&leaf),   
  ); 
}
```

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new( Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "1 leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
        );
    
    {
        let branch = Rc::new( Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!(
            "1 branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
            );

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "2 branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
            );

        println!(
            "2 leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
            );
    }

    println!(
        "3 branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
        );

    println!(
        "3 leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
        );
    
    println!("Hello, world!");
}
```







