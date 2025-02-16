## Trait

- [Trait](#trait)
- [basic usage](#basic-usage)
- [Polymorphism](#polymorphism)
  - [trait as parameter](#trait-as-parameter)
  - [trait as return](#trait-as-return)
  - [trait example](#trait-example)
  - [conditional trait bound, `impl<T>`](#conditional-trait-bound-implt)
- [implement `Display` for custom type](#implement-display-for-custom-type)
- [trait object](#trait-object)
- [`self` vs `Self`](#self-vs-self)
- [Polymorphism in Rust](#polymorphism-in-rust)
  - [Static Dispatch by `enum`](#static-dispatch-by-enum)
  - [Static Dispatch by `generics` with trait](#static-dispatch-by-generics-with-trait)
  - [Dynamic Dispatch `dyn` with trait object](#dynamic-dispatch-dyn-with-trait-object)
  - [Static Dispatch by `enum` with trait](#static-dispatch-by-enum-with-trait)

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
> `impl Trait`本质是trait bound的语法糖，适用于简单情形

```rs
pub fn notify2<T:Summary>(item1: T, item2: T) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}

pub fn notify3(item1: impl Summary, item2: impl Summary) {
    println!("Breaking New, {}", item1.summarize1());
    println!("Breaking New, {}", item2.summarize2());
}

pub fn notify4(item1: &impl Summary, item2: &impl Summary) {
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

pub fn notify4(item1: &(impl Summary+Display), item2: &(impl Summary+Display)) {
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
    // 只能返回一种类型NewsArticle或者Tweet，根据bool判断返回不同的类型，会报错
    NewsArticle{
        headline:String::from("Trump win!"),
        content:String::from("Trump win the 2024 election!"),
        author: String::from("Joe Biden"),
        location: String::from("Washington Dc"),
    }
}
```

error example: 返回多种类型

```rs
// error
pub fn returns_summarizable(flag: bool) -> impl Summary {
    if flag {
        NewsArticle {
            headline: String::from("Biden will win"),
            location: String::from("Washington"),
            author: String::from("Hilary"),
            content: String::from("Biden will win the election"),
        }
    } else {
        Tweet {
            username: String::from("Trump"),
            content: String::from("Trump will win"),
            reply: true,
            retweet: true,
        }
    }
}
```

solution: trait object

```rs
// lib.rs
pub fn returns_summarizable(flag: bool) -> Box<dyn Summary> {
    if flag {
        let item = NewsArticle {
            headline: String::from("Biden will win"),
            location: String::from("Washington"),
            author: String::from("Hilary"),
            content: String::from("Biden will win the election"),
        };
        Box::new(item)
    } else {
        let item = Tweet {
            username: String::from("Trump"),
            content: String::from("Trump will win"),
            reply: true,
            retweet: true,
        };
        Box::new(item)
    }
}
```

```rs
use project1::returns_summarizable;

fn main() {
    let summary1 = returns_summarizable(true); // 自动识别为Box<dyn Summary>类型
    println!("{:?}", summary1.summarize1());
    let summary2 = returns_summarizable(false);
    println!("{:?}", summary2.summarize1());
}
```

### trait example

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

example: 保持p1和p2的所有权, method1, 要求Copy trait, 只能处理stack上面的数据
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

example: 保持p1和p2的所有权, method2, 只使用reference

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

### conditional trait bound, `impl<T>`

```rs
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// impl<T>, 无论T是何种类型，都有new
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

// impl<T: Display + PartialOrd>, 只有T实现了Display + PartialOrd，才有一个cmp_display函数
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x is larger, {}", self.x);
        } else {
            println!("y is larger, {}", self.x);
        }
    }
}

fn main() {
    let p1 = Pair { x: 10, y: 20 };
    p1.cmp_display();
    let p2 = Pair {
        x: String::from("Trump"),
        y: String::from("Biden"),
    };
    p2.cmp_display();

    let p3 = Pair::new(12.5, 10.0); // 因为new没有self, 只能通过Pair::new调用
    p3.cmp_display();

    Pair::cmp_display(&p2);
}
```

**blanket implementations**:
> 为满足trait bound的类型实现某种trait, 叫做**覆盖实现**

对于标准库`string.rs`中, 

```rs
// 对于满足trait bound(Display)的类型T，实现了一个新的trait, 叫做ToString
impl<T: Display> ToString for T {
    // 这个ToString trait里面正好有一个to_string函数，实现数据转字符串的功能
    fn to_string(&self)->String{
        //....
    }
}
```

```rs
fn main() {
    let s1 = 3.to_string();
    let s2 = 12.5.to_string();
    let s3 = 'c'.to_string();
    let s4 = "hello".to_string();
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);
}
```

## implement `Display` for custom type

```rs
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match *self {
        match self { // 隐式解引用
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    //...
    println!("{}", f6); // <f6.txt (CLOSED)>
    println!("{:?}", f6); // File { name: "f6.txt", data: [], state: Closed }
}
```

## trait object

trait对象通过三种方式创建: `&dyn Trait`, `Box<dyn Trait>`, `Rc<dyn Trait>`

```rs
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        // format!("u8: {}", *self)
        format!("u8: {}", self) // 隐式解引用可以直接用self
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        // format!("f64: {}", *self)
        format!("f64: {}", self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    let result = x.draw();
    println!("{}", result);
}

fn draw2(x: &dyn Draw) {
    // Box<dyn Draw>强制在heap上; &dyn Draw只是一个借用，数据所有权不属于自己，数据可以在stack也可以在heap
    // Stacks tend to have a fixed size that is relatively small
    let result = x.draw();
    println!("{}", result);
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));

    draw2(&x);
    draw2(&y);
}
```

当一个特征的所有方法都有如下属性时，它的对象才是安全的：
- 方法的返回类型不能是 `Self`
- 方法没有任何泛型参数

## `self` vs `Self`

- `self`:指代的就是当前的实例对象, 比如`btn`
- `Self`: 指代的是实例对象的类型, 比如`Button`

```rs
trait Draw {
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;


impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone();
    }
}

fn main() {
    let btn = Button;
    let newb = btn.draw();
}
```

## Polymorphism in Rust

4 methods for [polymorphism in Rust](https://www.brandons.me/blog/polymorphism-in-rust) 
1. Static Dispatch by `enum`
1. Static Dispatch by `generics` with trait
1. Dynamic Dispatch `dyn` with trait object
2. Static Dispatch by `enum` with trait

|              | Inline layout | No wasted memory | Mixed-type collections | Extensibile | Easy to write and maintain |
| :----------: | :-----------: | :--------------: | :--------------------: | :---------: | :------------------------: |
|     enum     |       ✅       |        ❌         |           ✅            |      ❌      |             ✅              |
|   generics   |       ✅       |        ✅         |           ❌            |      ✅      |             ✅              |
|     dyn      |       ❌       |        ✅         |           ✅            |      ✅      |             ✅              |
| Frankenstein |       ✅       |        ✅         |           ✅            |      ✅      |             ❌              |

### Static Dispatch by `enum`

> like c++ `std::variant`

```rs
enum ShapeEnum {
    Rectangle { width: f32, height: f32 },
    Triangle { side: f32 },
    Circle { radius: f32 },
}

impl ShapeEnum {
    pub fn perimeter(&self) -> f32 {
        match self {
            ShapeEnum::Rectangle { width, height } => width * 2.0 + height * 2.0,
            ShapeEnum::Triangle { side } => side * 3.0,
            ShapeEnum::Circle { radius } => radius * 2.0 * std::f32::consts::PI,
        }
    }

    pub fn area(&self) -> f32 {
        match self {
            ShapeEnum::Rectangle { width, height } => width * height,
            ShapeEnum::Triangle { side } => side * 0.5 * 3.0_f32.sqrt() / 2.0 * side,
            ShapeEnum::Circle { radius } => radius * radius * std::f32::consts::PI,
        }
    }
}

// usage
fn print_area(shape: ShapeEnum) {
    println!("{}", shape.area());
}

fn print_perimeters(shapes: Vec<ShapeEnum>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn main() {
    let s1 = ShapeEnum::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    print_area(s1);

    let v1 = vec![
        ShapeEnum::Rectangle {
            width: 10.0,
            height: 30.0,
        },
        ShapeEnum::Triangle { side: 20.0 },
        ShapeEnum::Circle { radius: 10.0 },
    ];
    print_perimeters(v1);
}
```

### Static Dispatch by `generics` with trait

> like c++ concept

```rs
trait Shape {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

struct Rectangle {
    pub width: f32,
    pub height: f32,
}
struct Triangle {
    pub side: f32,
}
struct Circle {
    pub radius: f32,
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side * 3.0
    }
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        self.radius * 2.0 * std::f32::consts::PI
    }
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

// usage: Traits with generics
fn print_area<S: Shape>(shape: S) {
    println!("{}", shape.area());
}

fn print_perimeters<S: Shape>(shapes: Vec<S>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn main() {
    let s1 = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    print_area(s1);

    let v1 = vec![
        Triangle { side: 10.0 },
        Triangle { side: 20.0 },
        Triangle { side: 30.0 },
    ]; // same shape for vec
    print_perimeters(v1);
}
```

### Dynamic Dispatch `dyn` with trait object

> like c++ **vtable**

```rs
trait Shape {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

struct Rectangle {
    pub width: f32,
    pub height: f32,
}
struct Triangle {
    pub side: f32,
}
struct Circle {
    pub radius: f32,
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side * 3.0
    }
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        self.radius * 2.0 * std::f32::consts::PI
    }
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

// usage: Traits with dynamic dispatch
fn print_area(shape: &dyn Shape) {
    println!("{}", shape.area());
}

fn print_perimeters(shapes: Vec<&dyn Shape>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn main() {
    let s1 = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    print_area(&s1);

    let s2 = Circle { radius: 10.0 };
    let s3 = Triangle { side: 20.0 };
    let v1: Vec<&dyn Shape> = vec![&s1, &s2, &s3]; // same shape for vec
    print_perimeters(v1);
}
```

### Static Dispatch by `enum` with trait

> frankenstein approach combined the `enum` and `trait`

```rs
enum ShapeEnum {
    Rectangle(Rectangle),
    Triangle(Triangle),
    Circle(Circle),
}

struct Rectangle {
    pub width: f32,
    pub height: f32,
}
struct Triangle {
    pub side: f32,
}
struct Circle {
    pub radius: f32,
}

trait Shape {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

impl Shape for ShapeEnum {
    fn perimeter(&self) -> f32 {
        match self {
            ShapeEnum::Rectangle(rect) => rect.perimeter(),
            ShapeEnum::Triangle(tri) => tri.perimeter(),
            ShapeEnum::Circle(circ) => circ.perimeter(),
        }
    }
    fn area(&self) -> f32 {
        match self {
            ShapeEnum::Rectangle(rect) => rect.area(),
            ShapeEnum::Triangle(tri) => tri.area(),
            ShapeEnum::Circle(circ) => circ.area(),
        }
    }
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side * 3.0
    }
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        self.radius * 2.0 * std::f32::consts::PI
    }
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

// usage
fn print_area(shape: ShapeEnum) {
    println!("{}", shape.area());
}

fn print_perimeters(shapes: Vec<ShapeEnum>) {
    for shape in shapes.iter() {
        println!("{}", shape.perimeter());
    }
}

fn main() {
    let s1 = ShapeEnum::Rectangle(Rectangle {
        width: 10.0,
        height: 20.0,
    });
    print_area(s1);

    let s2 = ShapeEnum::Circle(Circle { radius: 10.0 });
    let s3 = ShapeEnum::Triangle(Triangle { side: 30.0 });
    let v1 = vec![s2, s3];
    print_perimeters(v1);
}
```