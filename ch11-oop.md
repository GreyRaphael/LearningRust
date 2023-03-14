# Object-Oriented Programming Features of Rust

- [Object-Oriented Programming Features of Rust](#object-oriented-programming-features-of-rust)
  - [oop features](#oop-features)
  - [`dyn`](#dyn)

## oop features

OOP特征
- 封装: public, private
- 继承: 使对象可以沿用另外一个对象的数据和行为。目前很多语言不使用继承作为程序内置的程序设计方案
  - 目的1： 代码复用。Rust没有继承，用trait方法来代码复用
  - 目的2： 实现多态(Polymorphism)。Rust使用泛型和tait bound实现多态

```rs
// lib.rs

// 封装，list, average外部不可见
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

## `dyn`

[What is dyn](https://doc.rust-lang.org/rust-by-example/trait/dyn.html)

```rs
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```

将trait约束作用于泛型时，Rust编译器会执行单态化
- 编译器会为泛型参数的每一个具体类型生成对应的函数和方法
- 通过单态化生成的代码会执行静态派发(**static dispatch**)，在编译过程中确定调用的具体方法

动态派发(**dynamic dispatch**)
- 无法在编译阶段确定调用的是哪一种方法
- 编译器会产生额外的代码以便在运行时找到希望调用的方法
- 使用trait对象会执行动态派发，产生运行时开销，阻止编译器内联方法代码，使得部分优化操作无法进行

```rs
// lib.rs
pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    // Vec要存各种实现了Draw trait的struct，所以使用Box<dyn Draw>>
    // 泛型只能放一种类型比如Button
    // Draw就是trait对象
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            // run不关心具体，只要实现了draw方法就行
            component.draw();
        }
    }
}

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label: String
}

impl Draw for Button{
    fn draw(&self){
        println!("drawn a button");
    }
}

pub struct SelectBox{
    pub width:u32,
    pub height:u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("drawn a select box");
    }
}

// // ---对比泛型实现-----------
// pub struct Screen<T:Draw>{
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where T: Draw
// {
//     pub fn run(&self){
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }
// // ------------------
```

```rs
// main.rs
use project1::{Button, SelectBox, Screen};

fn main() {
    let screen=Screen{
        components:vec![
            Box::new(SelectBox{
                width:75,
                height:10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Cancel"),
                ]
            }),
            Box::new(Button{
                width:50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };

    screen.run();
}
```

只有满足object-safe的trait才能转化为trait对象，判断规则
- 方法不返回`Self`
- 方法中不包含任何泛型类型参数

example: Clone trait因为返回值是`Self`, 所以无法转化为trait对象

```rs
pub trait Clone{
    fn clone(&self)->Self;
}

pub struct Screen{
    // pub components: Vec<Box<dyn Clone>>, // error
}
```