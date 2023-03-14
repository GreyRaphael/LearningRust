# Object-Oriented Programming Features of Rust

- [Object-Oriented Programming Features of Rust](#object-oriented-programming-features-of-rust)

OOP三大特征
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