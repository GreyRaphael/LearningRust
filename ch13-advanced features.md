# Advanced Features

- [Advanced Features](#advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [raw pointer](#raw-pointer)
    - [unsafe function](#unsafe-function)
    - [`extern`](#extern)
    - [global variable, `static`](#global-variable-static)
    - [unsafe trait](#unsafe-trait)
  - [Advanced Trait](#advanced-trait)
    - [Associated Types](#associated-types)
    - [Operator overloading](#operator-overloading)
    - [Calling Methods with the Same Name](#calling-methods-with-the-same-name)
    - [super trait](#super-trait)
    - [Implement External Traits on External Types](#implement-external-traits-on-external-types)
  - [Advanced Types](#advanced-types)
    - [Type Aliases](#type-aliases)
    - [Never Type `!`](#never-type-)
    - [Dynamically Sized Types(DST)](#dynamically-sized-typesdst)
  - [Advanced Functions and Closures](#advanced-functions-and-closures)
    - [Function Pointers](#function-pointers)
    - [Returning Closures](#returning-closures)
  - [macro](#macro)
    - [Declarative Macros](#declarative-macros)
    - [Procedural Macros](#procedural-macros)

## Unsafe Rust

Unsafe Rust存在原因
- 静态分析是保守的
- 计算机硬件本事是unsafe, Rust进行底层系统编程需要unsafe

Unsafe Rust四个功能
- 解引用原始指针(raw pointer)
- 调用unsafe函数或方法
- 访问或修改可变的静态变量
- 实现unsafe trait

### raw pointer

目的: 
- 与C语言进行接口
- 构建借用检查器无法理解的抽象

```rs
fn main(){
    let mut num=10;

    let raw_pointer1=&num as *const i32;
    let raw_pointer2=&mut num as *mut i32;
    
    unsafe{
        *raw_pointer2=2000;
        println!("value1={}", *raw_pointer1); // value1=2000
        println!("value2={}", *raw_pointer2); // value2=2000
    }

    let address=0x012345usize;
    let raw_pointer3= address as *const i32;
    unsafe {
        println!("value3={}", *raw_pointer3); // Segmentation fault (core dumped), illegal visit
    }
}
```

### unsafe function

```rs
fn main(){
    unsafe{
        dangerous();
    }
}

unsafe fn dangerous(){}
```

```rs
use std::slice;

// override split_at_mut
// 具有不安全代码的安全抽象
fn split_at_mut(a_slice: &mut[i32], mid: usize)->(&mut[i32], &mut[i32]){
    let length=a_slice.len();
    let raw_ptr=a_slice.as_mut_ptr();// *mut i32

    assert!(mid<=length);
    unsafe{
        (
            slice::from_raw_parts_mut(raw_ptr, mid),
            // .add(mid)以mid为偏移量的指针
            slice::from_raw_parts_mut(raw_ptr.add(mid), length-mid),
        )
    }
}

fn main(){
    let mut v=vec![1,2, 3, 4, 5, 6];
    let slice1=&mut v[..];
    let (a, b)=slice1.split_at_mut(3);
    assert_eq!(a, &mut[1, 2, 3]);
    assert_eq!(b, &mut[4, 5, 6]);
    println!("a={:?}, b={:?}", a, b); // a=[1, 2, 3], b=[4, 5, 6]

    // error example
    let addr=0x012345usize;
    let raw_ptr=addr as *mut i32;
    let slice2:&[i32]=unsafe {
        // slice::from_raw_parts_mut(raw_ptr, 10); // error
    };
}
```

### `extern`

作用：简化创建和使用外部函数接口(FFI, Foreign Function Interface)的过程
> 允许一种编程语言定义函数，并让其他编程语言调用这些函数  
> `extern`声明的函数都是`unsafe`

调用其他语言提供的函数

```rs
extern "C" { // 遵循C语言的ABI
    fn abs(input :i32)->i32;
}

fn main() {
    unsafe{
        println!("abs = {}", abs(-3));
    }
}
```

暴露rust函数给其他语言使用

```rs
#[no_mangle] // 不允许编译器改函数的名字
pub extern "C" fn call_from_c(){
    println!("Just call a function from C!");
}

fn main(){}
```

### global variable, `static`

Rust支持全局变量，但因为所有权机制可能产生数据竞争问题

```rs
// 生命周期默认都是'static, 无需显式标注
static G_NUM: &str="Biden";
static mut COUNTER: u32=0;

fn add_to_count(inc:u32){
    unsafe{
        COUNTER+=inc;
    }
}

fn main(){
    println!("name is: {}", G_NUM);

    add_to_count(3);
    unsafe{
        // 并行或者并发的时候不要使用静态变量
        println!("{}", COUNTER); // 3
    }
}
```

### unsafe trait

```rs
unsafe trait Foo{

}

unsafe impl Foo for i32{

}

fn main(){}
```

## Advanced Trait

### Associated Types

关联类型(Associated Types) vs 泛型
- 关联类型实现需要`type Item=u32`，而且只能针对`Conter`实现一种类型的trait(u32)
- 泛型实现需要标注`<u32>, <Strng>`，能够针对`Conter`实现多种种类型的trait(u32, String)

```rs
pub trait Iterator {
    type Item; // Item is Associated Types

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    // 泛型
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32; // 实现的过程需要标注 u32, 而且只能实现一次
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
}

fn main() {}
```

### Operator overloading

只能实现`std::ops`里面的运算符重载

```rs
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p1 + p2); // Point { x: 3, y: 3 }
}
```

其中`std::ops::Add` trait里面泛型指定了默认类型`<Rhs=Self>`

```rs
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

example: 两种不同类型相加

```rs
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let p1 = Millimeters(1000);
    let p2 = Meters(1);
    println!("{:?}", p1); // Millimeters(1000)
    println!("{:?}", p2); // Meters(1)
    println!("{:?}", p1 + p2); // Millimeters(2000)
}
```

### Calling Methods with the Same Name

example: 多个trait有同名method, 并且有`&self`参数的情形

```rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    person.fly(); // 调用自己的方法
    // 调用实现的trait的方法
    Pilot::fly(&person);
    Wizard::fly(&person);
}
```

example: 多个trait有同名method, 并且无参数的情形, 使用**Fully Qualified Syntax**

```rs
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("{}", Dog::baby_name()); // Spot
    // println!("{}", Animal::baby_name()); // error
    println!("{}", <Dog as Animal>::baby_name()); // puppy
}
```

### super trait

trait继承依赖其他的trait
- 被间接依赖的trait叫做**supertrait**
- 被间接依赖的trait也需要被实现

```rs
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

// 因为self.to_string,所以需要实现fmt::Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 100, y: 200 };
    p1.outline_print();
}
```

### Implement External Traits on External Types

[孤儿规则](ch04-trait.md#basic-usage): trait及类型，至少有一个在本地
> 通过`newtype`模式绕过该规则, 利用tuple struct创建一个新的类型

```rs
use std::fmt;

struct Wrapper(Vec<String>); // Display和Vec都是外部的trait或者类型

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0就是Vec
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

## Advanced Types

### Type Aliases

```rs
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
```

example: 优化教长的类型

```rs
// 泛型参数是函数F()，输入值为空，返回值为空
// Send用于异步
// 'static用于静态生命周期
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn main() {
    let f1: Thunk = Box::new(|| println!("hello"));
    takes_long_type(f1);

    let f2 = returns_long_type();
    f2();
}
```

### Never Type `!`

NeverType形式如下，绝对不会返回值，比如
- `continue`
- `panic`

```rs
fn bar() -> ! {
    // 报错，因为不写，返回值是()
}
```

### Dynamically Sized Types(DST)

只能在运行时才能确定大小的类型, 比如`str`(`&str`不是)

```rs
// error example
let s1: str = "Hello there!";
let s2: str = "How's it going?";

// 使用&str切片来解决：因为&str存：str地址, str长度

// correct example
let s3: &str = "Hello there!";
let s4: &str = "How's it going?";
```

每个trait都是一个动态大小的类型
> 使用trait对象，必须将其放在某种指针之后: `&dyn Trait`, `Box<dyn Trait>`, `Rc<dyn Trait>`

## Advanced Functions and Closures

### Function Pointers

函数指针
- 将函数作为参数传递给其他函数
- 被传递的函数转化成`fn`类型(functin pointer)
- `fn`是一个类型，不是一个`trait`, 可以直接指定fn为参数类型，不用声明一个以`Fn trait`为约束的泛型参数
- 并且`fn`实现了`Fn`, `FnMut`, `FnOnce`三种trait，所以只要是能够接受**闭包**的函数，都能接受**函数指针**

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let a = do_twice(add_one, 10);
    println!("{}", a); // 22
}
```

example: `map`接受的参数必须是实现了`FnMut` trait

```rs
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let strings: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", strings); // ["1", "2", "3", "4"]

    let another_strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", another_strings); // ["1", "2", "3", "4"]
}
```

```rs
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let v1 = Status::Value(10); // 长得像函数，本质也实现了FnMut trait
    let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("{:?}", list_of_statuses); // [Value(0), Value(1), Value(2), Value(3), Value(4)]
}
```

### Returning Closures

```rs
// // error: doesn't have a size known at compile-time
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// 因为trait作为对象必须放在某种指针后面，所以需要Box
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let f = returns_closure();
    println!("{}", f(10)); // 11
}
```

## macro

宏: 是用来编写可以生成其他代码的代码(metaprogramming)

### Declarative Macros

比如`vec!`, `println!`, `print!`

### Procedural Macros

```rs
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // 本质是代码作为TokenStream传入，处理之后变成代码TokenStream
}
```

example: custom procedural `derive` macros
- `derive`宏仅仅适用于enum, struct

```bash
# 需要创建一个workspace
mkdir workspace2
touch Cargo.toml
cargo new hello_macro --lib --vcs=none
cargo new hello_macro_derive --lib --vcs=none
cargo new pancakes --vcv=none

.
├── Cargo.toml
├── hello_macro
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── hello_macro_derive
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── pancakes
    ├── Cargo.toml
    └── src
        └── main.rs
```

```toml
# ./Cargo.toml
[workspace]

members=[
    "hello_macro",
    "hello_macro_derive",
    "pancakes",
]
```

```toml
# ./hello_macro_derive/Cargo.toml
[package]
name = "hello_macro_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro=true

[dependencies]
syn = "1.0.109"
quote = "1.0.26"
```

```toml
# ./pancakes/Cargo.toml
[package]
name = "pancakes"
version = "0.1.0"
edition = "2021"

[dependencies]
hello_macro={path="../hello_macro"}
hello_macro_derive={path="../hello_macro_derive"}
```

```rs
// ./hello_macro/src/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}
```

```rs
// ./hello_macro_derive/src/lib.rs
use proc_macro::TokenStream;
use syn; // 将rust代码字符串转化为可供操作的数据结构
use quote::quote; //将syn的数据结构重新转化为rust代码

// #[derive(HelloMacro)]的时候，hello_macro_derive就会被调用
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

```rs
// ./pancakes/src/main.rs
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Apple;

fn main() {
    Pancakes::hello_macro(); // Hello, Macro! My name is Pancakes!
    Apple::hello_macro(); // Hello, Macro! My name is Apple!
}
```