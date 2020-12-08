```rust
//1、trait用于定义与其它类型共享的功能，类似于其它语言中的接口。
//（1）可以通过trait以抽象的方式定义共享的行为。
//（2）可以使用trait bounds指定泛型是任何拥有特定行为的类型。
//2、定义trait
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("HongXingSchool")
    }
}

//3、实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl SchoolName for Student {}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl SchoolName for Teacher{
    fn get_school_name(&self) -> String {
        String::from("GuangmingSchool")
    }
}

impl GetInformation for Teacher{
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

//4、默认实现：可以在定义trait的时候提供默认的行为，trait的类型可以使用默认的行为。
trait SchoolName {
  fn get_school_name(&self) -> String {
    String::from("HongXingSchool")
  }
}
//下面使用默认实现
pub struct Teacher {
  pub name: String,
  pub age: u32,
}
impl SchoolName for Teacher {}

fn main() {
  let t = Teacher{ name: String::from("andy"), age: 32};
  let t_school_name = t.get_school_name();
  println!("teacher school name = {}", t_school_name);
}

//5、trait作为参数
fn print_information(item: impl GetInformation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    let t = Teacher{name: "xiaohuang".to_string(), age: 30, subject: String::from("math")};
    //println!("student, name = {}, age = {}", s.get_name(), s.get_age());
    //println!("teacher, name = {}, age = {}", t.get_name(), t.get_age());

    let s_school_name = s.get_school_name();
    println!("student school name = {}", s_school_name);
    let t_school_name = t.get_school_name();
    println!("teacher school name = {}", t_school_name);

    print_information(s);
    print_information(t);
    
    println!("Hello, world!");
}



//6、trait bound语法
//上面的例子可以写成：

pub fn print_information<T: GetInformation>(item: T) {
  println!("name = {}", item.get_name());
  println!("age = {}", item.get_age());
}

//7、通过+指定多个trait bound
pub trait GetName {
  fn get_name(&self) -> &String;
}

pub trait GetAge {
  fn get_age(&self) -> u32;
}

//写法一
pub fn print_information<T: GetName+GetAge>(item: T) {
  println!("name = {}", item.get_name());
  println!("age = {}", item.get_age());
}


//写法二，使用where
pub fn print_information<T>(item: T) where T: GetName+GetAge {
  println!("name = {}", item.get_name());
  println!("age = {}", item.get_age());
}

//8、返回trait的类型
fn produce_item_with_information() -> impl GetInformation {
  Teacher {
    name: String::from("Andy"),
    age: 32,
  }
}
//但是，需要注意的是，只能返回单一类型，以下实现会出错（因为返回了两个类型）：
fn produce_item_with_information(is_teacher: bool) -> impl GetInformation {
  if is_teacher {
    Teacher {
      name: String::from("Andy"),
      age: 32,
    }
  } else {
    Student {
      name: String::from("harden"),
      age: 47,
    }
  }
}

//9、复习之前的largest例子
fn largest<T：PartialOrd + Copy>(list: &[T]) -> T { //注意，要实现比较和复制的trait才行，否则报
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main(){
  let number_list = vec![1, 2, 22, 3, 42];
  let r1 = largest_i32(&number_list);
  println!("r1 = {}", r1);
  let char_list = vec!['a', 'y', 'c', 'd'];
  let r2 = largest_char(&char_list);
  println!("r2 = {}", r2);
}

//10、使用trait bound有条件地的实现方法

pub trait GetName {
  fn get_name(&self) -> &String;
}

pub trait GetAge {
  fn get_age(&self) -> u32;
}
struct PeopleMatchInformation<T, U> {
  master: T,
  employee: U,
}

impl <T: GetName+GetAge , U: GetName+GetAge> PeopleMatchInformation<T, U> {
  fn print_all_information(&self) {
    println!("teacher name = {}", self.master.get_name());
    println!("teacher age = {}", self.master.get_age());
    println!("student name = {}", self.employee.get_name());
    println!("student age = {}", self.employee.get_age());
  }
}
//使用
pub struct Teacher {
  pub name: String,
  pub age: u32,
}

impl GetName for Teacher {
  fn get_name(&self) -> &String {
    &(self.name)
  }
}

impl GetAge for Teacher {
  fn get_age(&self) -> u32{
    self.age
  }
}

pub struct Student{
  pub name: String,
  pub age: u32,
}

impl GetName for Student {
  fn get_name(&self) -> &String {
    &(self.name)
  }
}
impl GetAge for Student {
  fn get_age(&self) -> u32{
    self.age
  }
}

fn main() {
  let t = Teacher{ name: String::from("andy"), age: 32};
  let s = Student {name: String::from("harden"), age: 47};
  let m = PeopleMatchInformation{master: t, employee: s};
  m.print_all_information();
}

//11、对任何实现了特定trait的类型有条件的实现trait
//例如：
pub trait GetName {
  fn get_name(&self) -> &String;
}
pub trait PrintName {
  fn print_name(&self) ;
}

impl<T: GetName> PrintName for T {
  fn print_name(&self) {
    println!("name = {}", self.get_name());
  }
}

//----------使用----------------
pub struct Student{
  pub name: String,
}
impl GetName for Student {
  fn get_name(&self) -> &String {
    &(self.name)
  }
}

fn main() {
  let s = Student{name: String::from("Andy")};
  s.print_name(); //student实现了GetName trait，因此可是直接使用PrintName trait中的函数print_name
}

//引申：这种方式类似于c++里面的继承


```

