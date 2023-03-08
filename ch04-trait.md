## Trait

- [Trait](#trait)
- [basic usage](#basic-usage)
- [Polymorphism](#polymorphism)
  - [trait as parameter](#trait-as-parameter)
  - [trait as return](#trait-as-return)

> Trait: 告诉编译器，某种类型具有哪些并且可以与其它类型共享的功能。**抽象地定义共享行为**，与其他语言中的interface有点类似

> Trait bounds(约束): 泛型类型参数指定为实现了特定行为的类型。也就是：**泛型的类型参数实现了某些Trait**

## basic usage

```rs
trait Summary {
    // trait: 把方法签名放在一起，来定义实现某种目的所必须的一组行为
    fn summarize1(&self) -> String;// 只有方法签名，没有具体实现, 实现该trait的类型，必须提供具体的方法
    fn summarize2(&self) -> String;// 一个trait可以有多个方法
    fn summarize3(&self) -> String;// 以;结尾
}

fn main() {}
```

trait实现过程

```bash
src
├── lib.rs
└── main.rs
```

```rs
// lib.rs
pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为类型实现trait
impl Summary for NewsArticle {
    // 所有的方法都要实现
    fn summarize1(&self) -> String {
        format!("{},by{} ({})", self.headline, self.author, self.location)
    }
    fn summarize2(&self) -> String {
        format!("content={}", self.content)
    }
}

impl Summary for Tweet {
    fn summarize1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize2(&self) -> String {
        format!("content={}", self.content)
    }
}
```

```rs
// main.rs
// project1是Cargo.toml package下面的name
use project1::Summary;
use project1::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("Trump"),
        content: String::from("MAGA!"),
        reply: true,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize1());
    println!("tweet details: {}", tweet.summarize2());
}
```

可以在某个类型上实现trait的前提
1. trait在本地定义(比如trait Summary),类型是本地定义的(比如Tweet)
2. trait在本地定义(比如trait Summary)，类型是标准库的(比如Vector)
3. trait是标准库的(比如Display)，类型是本地定义的(比如Tweet)

example: trait默认实现, 类型实现可以覆盖，也可以不实现

```rs
pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
    fn summarize3(&self) -> String {
        String::from("default message")
    }
}
```

example: 默认实现可以调用其他实现

```rs
pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
    fn summarize3(&self) -> String {
        String::from("default message")
    }
    fn summarize4(&self) -> String {
        format!("info: {}", self.summarize2()) // 调用summarize2()
    }
}
```

```rs
// main.rs
fn main() {
    let tweet = Tweet {
        username: String::from("Trump"),
        content: String::from("MAGA!"),
        reply: true,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize1());
    println!("tweet details: {}", tweet.summarize2());
    println!("{}", tweet.summarize3());
    println!("{}", tweet.summarize4());
}
```

## Polymorphism

### trait as parameter

trait作为函数参数，实现多态
- 采用trait bound是更加通用的做法，适用于复杂情形，`notify<T: Summary>(item: T)`
- 比如下面的`notify(item: impl Summary)`，适用于简单的场景，只是trait bound的语法糖

```rs
// lib.rs
pub trait Summary {
    fn summarize1(&self) -> String;
    fn summarize2(&self) -> String;
    fn summarize3(&self) -> String {
        String::from("default message")
    }
    fn summarize4(&self) -> String {
        format!("info: {}", self.summarize2())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为类型实现trait
impl Summary for NewsArticle {
    // 所有的方法都要实现
    fn summarize1(&self) -> String {
        format!("{},by{} ({})", self.headline, self.author, self.location)
    }
    fn summarize2(&self) -> String {
        format!("content={}", self.content)
    }
}

impl Summary for Tweet {
    fn summarize1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize2(&self) -> String {
        format!("content={}", self.content)
    }
}

pub fn notify(item: impl Summary) { //参数为实现了Summary的类型
    println!("Breaking New, {}", item.summarize1());
}
```

```rs
use project1::notify;
use project1::Summary;
use project1::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("Trump"),
        content: String::from("MAGA!"),
        reply: true,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize1());
    println!("tweet details: {}", tweet.summarize2());
    println!("{}", tweet.summarize3());
    println!("{}", tweet.summarize4());

    notify(tweet);
}
```

example: trait bounds

```rs
// pub fn notify(item: impl Summary) {
//     println!("Breaking New, {}", item.summarize1());
// }

pub fn notify<T: Summary>(item: T) {
    println!("Breaking New, {}", item.summarize1());
}
```

复杂情形trait作为参数

```rs
pub fn notify2<T:Summary>(item1: T, item2: T) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}

pub fn notify3(item1: impl Summary, item2: impl Summary) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}
```

多个trait作为参数

```rs
pub fn notify2<T:Summary+Display>(item1: T, item2: T) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}

pub fn notify3(item1: impl Summary+Display, item2: impl Summary+Display) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}
```

采用`where`简化写法

```rs
pub fn notify4<T: Summary + Display, U: Clone + Debug>(a: T, b: U) {
    println!("Breaking New, {}", a.summarize1());
}

// 简化后
pub fn notify5<T, U>(a: T, b: U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
    println!("Breaking New, {}", a.summarize1());
}
```

### trait as return

```rs
pub fn notify6(s:&str)->impl Summary {
    // 只能返回一种类型，如果采用bool判断，进而返回不同的类型，会报错
    NewsArticle{
        headline:String::from("Trump win!"),
        content:String::from("Trump win the 2024 election!"),
        author: String::from("Joe Biden"),
        location: String::from("Washington Dc"),
    }
}
```