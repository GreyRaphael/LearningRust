## Trait

- [Trait](#trait)
- [basic usage](#basic-usage)
- [Polymorphism](#polymorphism)
  - [trait as parameter](#trait-as-parameter)
  - [trait as return](#trait-as-return)
  - [trait `PartialOrd`](#trait-partialord)

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

### trait `PartialOrd`

改造之前的`get_largest2`函数，使之能够处理`>`
> 如果容器里面的元素，实现了`Copy` trait，元素保存在stack, 那么可以使用下面的例子

```rs
fn get_largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // 通过+Copy trait解决
    for &item in list {
        if item > largest {
            // >的问题通过std::cmp::PartialOrd解决
            largest = item;
        }
    }
    largest
}

fn main() {
    let v1 = vec![11, 2, 33, 4, 5];
    let l1 = [11, 2, 3, 44, 5];
    let l2 = ['y', 'm', 'A', 'b'];
    let n1 = get_largest2(&v1);
    let n2 = get_largest2(&l1);
    let n3 = get_largest2(&l2);
    println!("{}", n1); // 33
    println!("{}", n2); // 44
    println!("{}", n3); // y
}
```

如果容器里面的元素保存在heap上(比如String)，该类型没有实现`Copy` trait, 参考下面的例子

```rs
fn get_largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone(); // 通过+Clone trait以及clone函数解决
    for item in list {
        if item > &largest {
            // >的问题通过std::cmp::PartialOrd解决
            largest = item.clone();
        }
    }
    largest
}

fn get_largest20<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            // >的问题通过std::cmp::PartialOrd解决
            largest = item;
        }
    }
    largest
}

fn main() {
    let l3 = [
        String::from("grey"),
        String::from("Trump"),
        String::from("Biden"),
    ];
    let n4 = get_largest2(&l3);
    let n5 = get_largest20(&l3);
    println!("{}", n4); // grey
    println!("{}", n5); // grey
}
```

保持p1和p2的所有权, method1, 要求Copy trait, 只能处理stack上面的数据
> 处理heap上面的数据，同理，需要使用`Clone`和`.clone()`

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: Copy, U> Point<T, U> {
    fn mixup<V, W: Copy>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // <&str, char>
    let p3 = p1.mixup(&p2); // 只需要限制T和W能够Copy就行
    println!("{}-{}", p1.x, p1.y); // 10-20.2
    println!("{}-{}", p2.x, p2.y); // hello-c
    println!("{}-{}", p3.x, p3.y); // 10-c
}
```

保持p1和p2的所有权, method2, 只使用reference

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 因为所有参数都是ref, 所有需要指定lifetime相同，使用'a
    // https://stackoverflow.com/questions/59522097/rust-lifetime-mismatch-in-trait-method
    fn mixup<'a, V, W>(&'a self, other: &'a Point<V, W>) -> Point<&T, &W> {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // <&str, char>
    let p3 = p1.mixup(&p2);
    println!("{}-{}", p1.x, p1.y); // 10-20.2
    println!("{}-{}", p2.x, p2.y); // hello-c
    println!("{}-{}", p3.x, p3.y); // 10-c
}
```