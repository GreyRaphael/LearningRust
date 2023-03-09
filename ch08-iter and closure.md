# Iterator and Closure

- [Iterator and Closure](#iterator-and-closure)
  - [Closure](#closure)

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