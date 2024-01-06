# Iterator and Closure

- [Iterator and Closure](#iterator-and-closure)
  - [Closure](#closure)
    - [Closure with generic](#closure-with-generic)
    - [Closure capture](#closure-capture)
    - [Closure as return](#closure-as-return)
  - [Iterator](#iterator)
    - [`Adapter` and `Consumer`](#adapter-and-consumer)
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
let sum = |x: i32, y: i32| -> i32 {
    x + y
}
```

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

example: 改造Cacher仅仅支持`Fn(u32)->u32`闭包的情况，使它能够支持`Fn(E)->E`类型闭包

```rs
struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    calc: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(calc: T) -> Cacher<T, E> {
        Cacher { calc, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut c = Cacher::new(|a| a + 1);

    let v1 = c.value(1);
    println!("{}", v1);

    let mut c = Cacher::new(|a| a * 2.0 + 1.0);
    let f1 = c.value(10.0);
    println!("{:?}", f1);
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

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：不可变引用，可变引用，转移所有权，因此相应的 Fn 特征也有三种：`Fn`, `FnMut`, `FnOnce`

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

从源码的特征约束来看，所有实现`Fn`的都实现了`FnMut`，所有实现了`FnMut`的都实现了`FnOnce`
> Fn 获取 `&self`(不可变引用)，FnMut 获取 `&mut self`(可变引用)，而 FnOnce 获取 `self`(转移所有权)。 在实际项目中，建议先使用 Fn 特征

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

实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
1. 取得所有权: `FnOnce`, applies to closures that can be called once. **All closures implement at least this trait**, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.(所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次)
1. 可变借用: `FnMut`，applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.(没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征)
1. 不可变借用: `Fn`，applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. (不需要对捕获变量进行改变的闭包自动实现了 Fn 特征)

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

迭代器: 不使用索引，而能遍历集合的工具(by `next()`)
> Rust只能通过迭代器访问集合，不能通过索引。

```rs
fn main() {
    {
        let v = vec![1, 2, 3, 4, 5];

        let it = v.iter(); // 创建不可变引用的迭代器Iter<'_, i32>
        for i in it {
            print!("{},", i); // i:&i32
        }
        println!()
    }
    {
        let mut v = vec![1, 2, 3, 4, 5];

        let it = v.iter_mut(); // 创建可变引用的迭代器IterMut<'_, i32>
        for i in it {
            *i += 1; // i: &mut i32
        }

        println!("{:?}", v);
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        let it = v.into_iter(); // 创建获取所有权的迭代器IntoIter<i32>
        for i in it {
            print!("{},", i); // i:i32
        }
        println!();
        // println!("{:?}", v); // error, it已经获取v的所有权
    }
}
```

所有的迭代器(`Iter`, `IterMut`, `IntoIter`)都实现了`Iterator` trait，进而能够被`for`循环遍历
- 实现Iterator trait需要定义一个Item类型，它用于`next`方法的返回类型(迭代器的返回类型)
- Iterator trait仅仅要求实现`next`方法，返回结果包裹在`Some`里面，迭代结束，返回`None`, 其他实现比如`map()`, `zip()`, `filter()`都自动被Rust实现

```rs
pub trait Iterator{
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}
```

数组、Range, 动态数组Vec, HashMap
1. 因为实现了`IntoIterator` trait, 那么它们可以调用`.iter()`, `.iter_mut()`, `.into_iter()`变成迭代器`Iter`, `IterMut`, `IntoIter`
1. 因为`Iter`, `IterMut`, `IntoIter`都实现了`Iterator` trait，所以可以使用`.next()`，进而能够通过`for`循环遍历
1. Rust采用语法糖，可以直接使用`for`循环遍历数组、Range, 动态数组Vec, HashMap


```rs
use std::collections::HashMap;

fn main() {
    // 数组
    let arr = [11, 22, 33, 44];
    for i in arr {
        print!("{} ", i);
    }
    println!();

    // Range<i32>
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();

    let r = 'a'..'e'; // Range<char>
    for i in r {
        print!("{} ", i);
    }
    println!();

    // 动态数组
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        print!("{} ", i);
    }
    println!();

    // HashMap
    let m = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    for (k, v) in m {
        print!("{}:{} ", k, v);
    }
    println!()
}
```

更加Iterator源码

```rs
pub trait Iterator{
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}
```

- `.iter()` 方法实现的迭代器，调用 next 方法返回的类型是 `Some(&T)`
- `.iter_mut()` 方法实现的迭代器，调用 next 方法返回的类型是 `Some(&mut T)`
- `.into_iter()` 方法实现的迭代器，调用 next 方法返回的类型是 `Some(T)`

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

### `Adapter` and `Consumer`

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

custom `xxxIter` iterator and `iter()`

```rs
#[derive(Debug)]
struct Todo {
    message: String,
    done: bool,
}

struct Todos {
    list: Vec<Todo>,
}

struct TodosIter<'a> {
    todos: &'a Todos,
    index: usize,
}

impl<'a> Iterator for TodosIter<'a> {
    type Item = &'a Todo;

    fn next(&mut self) -> Option<Self::Item> {
        let todo = self.todos.list.get(self.index);
        self.index += 1;
        todo
    }
}

impl Todos {
    fn iter(&self) -> TodosIter {
        TodosIter {
            todos: self,
            index: 0,
        }
    }
}

fn main() {
    let todos = Todos {
        list: vec![
            Todo {
                message: "Buy milk".to_string(),
                done: false,
            },
            Todo {
                message: "Buy eggs".to_string(),
                done: false,
            },
            Todo {
                message: "Buy bread".to_string(),
                done: false,
            },
        ],
    };
    // 无法使用for语法糖
    for todo in todos.iter() {
        println!("{:?}", todo); // &Todo
    }
}
```

custom `xxxIntoIter` iterator and `into_iter()`

```rs
#[derive(Debug)]
struct Todo {
    message: String,
    done: bool,
}

#[derive(Debug)]
struct Todos {
    list: Vec<Todo>,
}

struct TodosIntoIter {
    todos: Todos,
}

impl Iterator for TodosIntoIter {
    type Item = Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.todos.list.len() == 0 {
            return None;
        }
        let result = self.todos.list.remove(0);
        Some(result)
    }
}

impl IntoIterator for Todos {
    type Item = Todo;
    type IntoIter = TodosIntoIter;

    fn into_iter(self) -> TodosIntoIter {
        TodosIntoIter { todos: self }
    }
}

fn main() {
    {
        let todos = Todos {
            list: vec![
                Todo {
                    message: "Buy milk".to_string(),
                    done: false,
                },
                Todo {
                    message: "Buy eggs".to_string(),
                    done: false,
                },
                Todo {
                    message: "Buy bread".to_string(),
                    done: false,
                },
            ],
        };
    
        for todo in todos.into_iter() {
            println!("{:?}", todo); // Todo
        }
        // println!("{:?}", todos);// error, 失去所有权
    }
    {
        let todos = Todos {
            list: vec![
                Todo {
                    message: "Buy milk".to_string(),
                    done: false,
                },
                Todo {
                    message: "Buy eggs".to_string(),
                    done: false,
                },
                Todo {
                    message: "Buy bread".to_string(),
                    done: false,
                },
            ],
        };
    
        // 可以直接使用for语法糖
        for todo in todos {
            println!("{:?}", todo); // Todo
        }
        // println!("{:?}", todos);// error, 失去所有权
    }
}
```

custom `xxxIterMut` iterator and `iter_mut()`

```rs
#[derive(Debug)]
struct Todo {
    message: String,
    done: bool,
}

#[derive(Debug)]
struct Todos {
    list: Vec<Todo>,
}

struct TodosIterMut<'a> {
    todos: &'a mut Todos,
    index: usize,
}

impl<'a> Iterator for TodosIterMut<'a> {
    type Item = &'a mut Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.todos.list.len() {
            let item = &mut self.todos.list[self.index] as *mut _;
            self.index += 1;
            Some(unsafe { &mut *item })
        } else {
            None
        }
    }
}

impl Todos {
    fn iter_mut(&mut self) -> TodosIterMut {
        TodosIterMut {
            todos: self,
            index: 0,
        }
    }
}

fn main() {
    let mut todos = Todos {
        list: vec![
            Todo {
                message: "Buy milk".to_string(),
                done: false,
            },
            Todo {
                message: "Buy eggs".to_string(),
                done: false,
            },
            Todo {
                message: "Buy bread".to_string(),
                done: false,
            },
        ],
    };

    for todo in todos.iter_mut() { // &mut Todo
        todo.done = true;
        todo.message= todo.message.to_uppercase();
    }
    println!("{:?}", todos);
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