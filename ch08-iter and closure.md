# Iterator and Closure

- [Iterator and Closure](#iterator-and-closure)
  - [Closure](#closure)
    - [Closure with generic](#closure-with-generic)
    - [Closure capture](#closure-capture)
    - [Closure as return](#closure-as-return)
  - [Iterator](#iterator)
    - [`Consumer` \& `Adapter`](#consumer--adapter)
    - [custom iterator](#custom-iterator)
    - [modify `minigrep` with iterator](#modify-minigrep-with-iterator)
    - [Iterator as function argument](#iterator-as-function-argument)

## Closure

闭包: 一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
- 匿名函数
- 保存为变量或者参数，传递给另一个函数，或者作为返回值
- 在某一个地方创建闭包，在另一个上下文中调用闭包来完成计算
- 可以从其定义的作用域内捕获值

```rs
fn main() {
   let x = 1;
   let mysum = |y| x + y; // mysum is closure

    assert_eq!(3, mysum(2));
}
```

```rs
// 完整形式
|param1, param2,...| {
    语句1;
    语句2;
    返回表达式
}

// 简单形式
|param1| 返回表达式
```

- 闭包不要求标注参数和返回值的类型, 闭包的上下文比较狭小，通常编译器都能推断出类型
- 可以手动添加类型标注
- 闭包的类型一旦被确定，就不能改变了

```rs
use std::{thread, time::Duration};

fn main() {
    generate_workout(60, 2);
}

fn generate_workout(intensity: u32, rand_num: u32) {
    // let expensive_closure = |num:u32| {
    let expensive_closure = |num| {
        println!("calc slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break today!");
        } else {
            println!("Today, run {} minuts", expensive_closure(intensity));
        }
    }
}
```

```rs
fn main() {
    let closure1 = |x| x;
    let s = closure1(String::from("hello"));
    // let num = closure1(10); //error, 闭包的类型一旦被确定，就不能改变了
}
```

### Closure with generic

上文这个地方，耗时计算了2次，解决方法1: 用一个变量存储`expensive_closure(intensity)`这个结果

```rs
if intensity < 25 {
    println!("Today, do {} pushups!", expensive_closure(intensity));
    println!("Next, do {} situps!", expensive_closure(intensity));
```

解决方法2: 创建一个struct，它持有闭包以及调用结果
- 只会在需要结果时才执行闭包
- 可缓存结果，也叫做记忆化(memoization)或者延迟计算(lazy evalution)

struct的定义需要知道所有字段的类型
- 每个闭包都有自己唯一的匿名类型，即使两个闭包签名一样
- 所以需要使用泛型和Trait Bound

`Fn Trait`由标准库提供，所有闭包至少实现了以下trait之一
- `Fn`
- `FnMut`
- `FnOnce`

```rs
use std::{thread, time::Duration};

fn main() {
    generate_workout(60, 2);

    let mut clo1 = Cacher::new(|x| x); // 闭包，输出参数x等于输出参数x
    let v1 = clo1.value(1);
    let v2 = clo1.value(2);
    println!("{}", v1); // 1
    println!("{}", v2); // 1, 闭包的值没有更新，采用HashMap改造value即可
}

fn generate_workout(intensity: u32, rand_num: u32) {
    let mut expensive_closure = Cacher::new(|num: u32| {
        println!("calc slowly...");
        thread::sleep(Duration::from_secs(5));
        intensity
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break today!");
        } else {
            println!("Today, run {} minuts", expensive_closure.value(intensity));
        }
    }
}

struct Cacher<T>
// bound, 接收的参数是函数类型，函数输入的是u32, 函数返回值是u32
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

改造数值不更新的问题

```rs
struct Cacher<T>
// bound, 接收的参数是函数类型，函数输入的是u32, 函数返回值是u32
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let mut clo1 = Cacher::new(|x| x); // 闭包，输出参数x等于输出参数x
    let v1 = clo1.value(1);
    let v2 = clo1.value(2);
    println!("{}", v1); // 1
    println!("{}", v2); // 2
}
```

### Closure capture

闭包能够访问定义它的作用域的变量，而普通函数不行
> 这个捕获的过程会产生额外的内存开销

```rs
fn main() {
    let x = 4;
    let clos1 = |z| z == x;

    // // 函数无法访问x，但是闭包可以访问x
    // fn func(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;
    assert!(clos1(y));
}
```

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种：
> 所有实现`Fn`的都实现了`FnMut`，所有实现了`FnMut`的都实现了`FnOnce`: `FnOnce > FnMut > Fn`
- 取得所有权: `FnOnce`, applies to closures that can be called once. **All closures implement at least this trait**, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- 可变借用: `FnMut`，applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
- 不可变借用: `Fn`，applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

example: `FnOnce`
> 闭包会拿走被捕获变量的所有权, 所以闭包只能运行Once

```rs
fn fn_once<F>(f_inner: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", f_inner(3));
    println!("{}", f_inner(4)); // f_inner转移所有权，内部第二次调用，直接报错
}

fn main() {
    let x = vec![1, 2, 3];
    let f_outer = |z| z == x.len(); // f_outer: impl Fn(usize)->bool
    fn_once(f_outer);

    println!("{}", f_outer(3)); // 因为f是Fn,所以可以再次调用
    println!("{}", f_outer(5));
}
```

```rs
fn fn_once<F>(f_inner: F)
where
    F: FnOnce(usize) -> bool + Copy, // 增加Copy trait
{
    println!("{}", f_inner(3));
    println!("{}", f_inner(4)); // 因为Copy trait调用时使用的将是它的拷贝，所以并没有发生所有权的转移, 正常运行
}

fn main() {
    let x = vec![1, 2, 3];
    let f_outer = |z| z == x.len(); // f_outer: impl Fn(usize)->bool
    fn_once(f_outer);

    println!("{}", f_outer(3)); // 因为f是Fn,所以可以再次调用
    println!("{}", f_outer(5));
}
```

example: `move`
> 闭包在参数列表前使用`move`关键字，可以强制取得它在所使用环境的控制权


```rs
fn fn_once<F>(f_inner: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", f_inner(3));
    println!("{}", f_inner(4)); // f_inner转移所有权，内部第二次调用，直接报错
}

fn main() {
    let x = vec![1, 2, 3];
    let f_outer = move |z| z == x.len(); // move强制取得所有权
    fn_once(f_outer);

    // println!("{}", f_outer(3)); // 因为move强制取得x的所有权，所以下面都无法调用
    // println!("{}", f_outer(5));
}
```

example: 当将闭包传递给新线程，以移动数据使其归新线程所有，`move`最为有用

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // println!("{:?}", v); // v失去所有权
    handle.join().unwrap();
}
```

example: `FnMut` by `mut`
> 以可变借用的方式捕获了环境中的值，因此可以修改该值

```rs
fn main() {
    let mut x = Vec::new();
    
    // FnMut需要用mut来修饰
    let mut update_vec = |value| x.push(value); // update_vec: impl FnMut(i32)
    update_vec(10);
    update_vec(20);

    println!("{:?}", x); // [10, 20]
}
```

example: `FnMut` by trait
> `update_vec`闭包的所有权被移交给了exec函数。这说明`update_vec`没有实现Copy特征，但并不是所有闭包都没有实现Copy特征，闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征。

```rs
fn main() {
    let mut x = Vec::new();

    // 没有mut修饰
    let update_vec = |value| x.push(value);

    exec(update_vec);
    // exec(update_vec); // update_vec moved

    println!("{:?}", x); // [11, 22]
}

// f使用mut修饰
fn exec<F: FnMut(usize)>(mut f: F) {
    f(11);
    f(22);
}
```

example: comparison

```rs
fn main() {
    let s = String::from("hello");
    let update_string = || println!("{}", s); // impl Fn()

    // 取得的是s的不可变引用，所以是能Copy的
    exec(update_string);
    exec(update_string);
}

// fn exec<F: Fn()>(f: F) // 也可以是Fn
// fn exec<F: FnOnce()>(f: F) // 也可以是FnOnce，但是内部的f只能调用1次
fn exec<F: FnMut()>(mut f: F) {
    f();
}
```

> **一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们。** 使用了不可变引用，就是`impl Fn()`, 使用了可变引用就是`impl FnMut()`, `move`并不改变Fn特征类型，只是表明闭包以`move`的方式捕获了变量。

```rs
fn main() {
    let s = String::from("hello");
    let update_string = move || println!("{}", s); // move, impl Fn()

    exec(update_string);
    // exec(update_string); // update_string moved，所以不是Copy
}

// fn exec<F: Fn()>(f: F) // 也可以是Fn
// fn exec<F: FnOnce()>(f: F) // 也可以是FnOnce，但是内部的f只能调用1次
fn exec<F: FnMut()>(mut f: F) {
    f();
}
```

```rs
fn main() {
    let mut s = String::new();
    let mut update_string = || s.push_str("hello"); // impl FnMut()

    exec(update_string);
    // exec(update_string); // 因为是可变引用mut s，所以不是Copy

    println!("{}", s);
}

// fn exec<F: Fn()>(f: F) {// 不能是Fn, 因为update_string类型推断是FnMut，根据FnOnce>FnMut>Fn, 只能选择FnMut, FnOnce
// fn exec<F: FnOnce()>(f: F) {// 也可以是FnOnce，但是内部的f只能调用1次
fn exec<F: FnMut()>(mut f: F) {
    f();
}
```

```rs
fn main() {
    let mut s = String::new();
    let update_string = || s.push_str("hello");

    exec(update_string);
    // exec(update_string); // 因为是可变引用mut s，所以不是Copy

    println!("{}", s);
}

// fn exec<F: Fn()>(f: F) {// 不能是Fn, 因为update_string类型推断是FnMut，根据FnOnce>FnMut>Fn, 只能选择FnMut, FnOnce
// fn exec<F: FnOnce()>(f: F) {// 也可以是FnOnce，但是内部的f只能调用1次
fn exec<F: FnMut()>(mut f: F) {
    f();
}
```

example: `Fn`

```rs
use std::usize;

fn main() {
    let mut x = Vec::new();

    let update_vec = |value| x.push(value); // imple FnMut(usize)

    exec(update_vec); // error, update_vec因为捕获的是可变引用，所以是FnMut，无法传递给Fn

    println!("{:?}", x);
}

fn exec<F: Fn(usize)>(mut f: F) {
    f(10);
    f(20);
}
```

```rs
fn main() {
    let s = "hello".to_string();

    let update_string = |ss| println!("{},{}", s, ss);

    exec(update_string);
    exec(update_string);

    println!("{:?}", s);
}

fn exec<F: Fn(String) -> ()>(f: F) {
    f("world".to_string())
}
```

example: 三种Fn特征的关系

从特征约束能看出来 Fn 的前提是实现 FnMut，FnMut 的前提是实现 FnOnce，因此要实现 Fn 就要同时实现 FnMut 和 FnOnce
> Fn 获取 `&self`，FnMut 获取 `&mut self`，而 FnOnce 获取 `self`。 在实际项目中，建议先使用 Fn 特征

```rs
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
1. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
1. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征

```rs
fn main() {
    let s = String::from("hello");

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}
```

关于规则2，有如下例子

```rs
fn main() {
    let mut s = String::new();

    let update_string = |seg| -> String {
        s.push_str(seg);
        s // 因为这里移出所捕获变量的所有权，所以是FnOnce
    }; // impl FnOnce(&str)->String

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) {
    let v = f("hello");
    println!("{}", v)
}
```

### Closure as return

```rs
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn main() {
    let func1 = factory(10);
    let func2 = factory(0);

    println!("{}", func1(12)); // 17
    println!("{}", func2(12)); // 7
}
```

## Iterator

- `iter`: 迭代不可变引用
- `into_iter`: 创建的迭代器会获取所有权
- `iter_mut`: 迭代可变引用

所有的迭代器都实现了`Iterator` trait，定义大致是
- 实现Iterator trait需要定义一个Item类型，它用于`next`方法的返回类型(迭代器的返回类型)
- Iterator trait仅仅要求实现`next`方法，返回结果包裹在`Some`里面，迭代结束，返回`None`

```rs
pub trait Iterator{
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}
```

simple example

```rs
fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let iter1 = v1.iter();
    for i in iter1 {
        // &i32
        print!("{},", i);
    }
    // vec实现了IntoIterator trait
    for i in v1 {
        // i32
        print!("{},", i);
    }
    println!("");

    let v2 = vec![11, 22, 33];
    let iter21 = v2.into_iter();
    for i in iter21 {
        // i32
        print!("{},", i);
    }
    println!("");

    let mut v3 = vec![100, 200, 300];
    let iter31 = v3.iter_mut();
    for i in iter31 {
        // &mut i32
        *i = *i - 10;
    }
    for i in v3 {
        // i32
        print!("{},", i);
    }
    println!("");
}
```

```rs
fn main() {
    let range1 = 0..5; // Range<i32>
    // Range实现IntoIterator trait
    for i in range1 {
        print!("{},", i)
    }
}
```

```rs
fn main() {
    let v1 = vec![1, 2, 3];
    let mut it1 = v1.iter(); //  手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成

    assert_eq!(it1.next(), Some(&1));
    assert_eq!(it1.next(), Some(&2));
    assert_eq!(it1.next(), Some(&3));
    assert_eq!(it1.next(), None);
}
```

example: 模拟for循环

```rs
fn main() {
    let values = vec![1, 2, 3];

    {
        match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => {
                        println!("{}", x);
                    }
                    None => break,
                }
            },
        };
    }
}
```

`Iterator` vs `IntoIterator`
- `Iterator` 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 `next`
- 而 `IntoIterator` 强调的是某一个类型(比如`Vec`)如果实现了该特征，它可以通过 `into_iter`，`iter`, `iter_mut()`变成一个迭代器。

### `Consumer` & `Adapter`

消费者适配器(`Consumer`): 只要迭代器上的某个方法 A 在其内部调用了 `next` 方法，那么 A 就被称为消费性适配器：因为 `next` 方法会消耗掉迭代器上的元素，所以方法 A 的调用也会消耗掉迭代器上的元素, 比如`.sum()`, `.collect()`
> Consumers take an iterator and return something other than an iterator, consuming the iterator in the process.

```rs
//                    Iterator  Adapter       Consumer
//                        |       |              |
let my_squares: Vec<_> = (1..6).map(|x| x * x).collect();
println!("{:?}", my_squares);
```

```rs
// sum源码
fn sum<S>(self) -> S // self获取了所有权
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        Sum::sum(self)
    }
```

```rs
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // 耗尽迭代器
    println!("{}", total); //6

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权, 无法再使用v1_iter
    // println!("{:?}",v1_iter);
}
```

迭代器适配器(`Adapter`)
> Adapters take an iterator and return another iterator, 比如`.map()`, `.zip()`, `.filter()`
- map接收一个闭包，闭包作用于每个元素
- 最终组成一个新的迭代器

```rs
//         Iterator  Adapter
//             |       |
let my_map = (1..6).map(|x| x * x);
println!("{:?}", my_map);
```

```rs
fn main() {
    let v1 = vec![1, 2, 3];
    let result: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", result); // [2, 3, 4]
}
```

`filter`

```rs
fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let fittable_shoes = shoes_in_my_size(shoes, 10);
    println!("{:?}", fittable_shoes); // [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}
```

`enumerate` adapter

```rs
fn main() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v
        .iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);

    println!("{}", val);
}
```

### custom iterator

```rs
fn main() {
    let c = Counter::new();
    for i in c {
        print!("{}, ", i); // 1, 2, 3, 4, 5,
    }

    using_other_iterator_trait_methods(); //18
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn using_other_iterator_trait_methods() {
    let result: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", result);
}
```

### modify `minigrep` with iterator

iterator改造[minigrep](ch07-io.md#test-driven-development)例子

```rs
// main.rs
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // env::args()本身就是iterator，函数new取得iter所有权，进而可以取得输入参数的所有权
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("config={:?}", config);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

```rs
// lib.rs
impl Config {
    // 因为错误信息是字符串字面值，所以用'static
    // 也可以使用了特征约束的方式来约束args
    // pub fn new(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str>
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next(); // 忽略输入的第一个元素
        let query = match args.next() {
            Some(x) => x,
            None => return Err("didnot get a query string"),
        };
        let filename = match args.next() {
            Some(x) => x,
            None => return Err("didnot get a filename string"),
        };

        // 如果这个环境变量CASE_INSENSITIVE出现，那么is_err()就是alse; 不出现就是true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // contenst.lines()本身也是iterator
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

### Iterator as function argument

```rs
fn iter1<'a>(numbers: impl Iterator<Item = &'a i32>) {
    for n in numbers {
        println!("{},", n);
    }
}

fn iter2(numbers: impl Iterator<Item = i32>) {
    for n in numbers {
        println!("{},", n);
    }
}

// iter3和iter2等效
fn iter3<T>(numbers: T)
where
    T: Iterator<Item = i32>,
{
    for n in numbers {
        println!("{},", n * 100);
    }
}

fn main() {
    // Range as Iter
    let range1 = 0..5;
    // iter1(range1.into_iter()); // expect &i32
    iter2(range1.into_iter()); // expect &i32
    // iter3(range1.into_iter()); // use of moved value: range1

    let range2 = 10..15;
    iter3(range2.into_iter());

    // Vector as Iter
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    iter1(numbers.iter()); // iter() element is &i32,后文能够使用numbers
    iter3(numbers.into_iter()); // into_iter之后，无法在后文使用numbers
}
```