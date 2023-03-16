# Iterator and Closure

- [Iterator and Closure](#iterator-and-closure)
  - [Closure](#closure)
    - [Closure with generic](#closure-with-generic)
    - [Closure capture](#closure-capture)
  - [Iterator](#iterator)
    - [custom iterator](#custom-iterator)
    - [modify `minigrep` with iterator](#modify-minigrep-with-iterator)

闭包: 可以捕获其所在环境的匿名函数
- 匿名函数
- 保存为变量或者参数，传递给另一个函数，或者作为返回值
- 在某一个地方创建闭包，在另一个上下文中调用闭包来完成计算
- 可以从其定义的作用域内捕获值

## Closure

- 闭包不要求标注参数和返回值的类型
- 闭包的上下文比较狭小，通常编译器都能推断出类型
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

与函数获取参数的方式一样, 创建闭包时，Rust会推断出具体使用下面的哪种闭包
> 所有实现`Fn`的都实现了`FnMut`，所有实现了`FnMut`的都实现了`FnOnce`
- 取得所有权: `FnOnce`, applies to closures that can be called once. **All closures implement at least this trait**, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- 可变借用: `FnMut`，applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
- 不可变借用: `Fn`，applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

闭包在参数列表前使用`move`关键字，可以强制取得它在所使用环境的控制权
- 当讲闭包传递给新线程，以移动数据使其归新线程所有，最为有用

```rs
fn main() {
    let x = vec![1, 2, 3];
    let clos1 = move |z| z == x;
    // println!("{}", x);// x失去所有权
}
```

## Iterator

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
    let v1 = vec![1, 2, 3, 4];
    for item in v1 {
        // i32
        println!("{}", item);
    }

    let v2 = vec![11, 22, 33, 44];
    let it2 = v2.iter();
    for item in it2 {
        // &i32
        println!("{}", item);
    }
}
```

```rs
fn main() {
    let v1 = vec![1, 2, 3];
    let mut it1 = v1.iter(); //  必须是mut，因为消耗迭代器，内部发生了改变, 上面的for是因为取得了所有权

    assert_eq!(it1.next(), Some(&1));
    assert_eq!(it1.next(), Some(&2));
    assert_eq!(it1.next(), Some(&3));
    assert_eq!(it1.next(), None);
}
``` 

summary：
- `iter`: 迭代不可变引用
- `into_iter`: 创建的迭代器会获取所有权
- `iter_mut`: 迭代可变引用

消耗迭代器

```rs
fn main() {
    let v1 = vec![1, 2, 3];
    let mut it1 = v1.iter();
    let total: i32 = it1.sum(); // 耗尽迭代器
    println!("{}", total); //6
}
```

产生其他迭代器，比如map
- map接收一个闭包，闭包作用于每个元素
- 最终组成一个新的迭代器

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