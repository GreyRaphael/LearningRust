## Trait

- [Trait](#trait)
- [basic usage](#basic-usage)

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