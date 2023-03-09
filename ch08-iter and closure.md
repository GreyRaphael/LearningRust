# Iterator and Closure

- [Iterator and Closure](#iterator-and-closure)
  - [Closure](#closure)
    - [Closure with generic](#closure-with-generic)

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