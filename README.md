# LearningRust

> Rust Learning Notes

- [LearningRust](#learningrust)
  - [Preparation](#preparation)
    - [First Example](#first-example)
    - [Cargo](#cargo)
  - [Guess Game](#guess-game)
  - [Code management](#code-management)
    - [Path](#path)
    - [`use`](#use)
    - [Package](#package)
    - [module to different files](#module-to-different-files)

## Preparation

```bash
sudo apt update
sudo apt install rustc
sudo apt install cargo
sudo apt install rust-src
# format rust code
sudo apt install rustfmt

rustc --version
cargo --version

# install vscode extension: rust-analyzer
```

### First Example

```rs
fn main() {
    println!("hello, world");
    println!("hello, 你好");
}
```

```bash
rustc hello_world.rs
./hello_world
```

### Cargo

```bash
# create new project
cargo new hello_cargo
# cargo new hello_cargo --vcs=none

tree .
.
├── Cargo.toml
└── src
    └── main.rs

# build & run
cargo run
# 检查代码，确保能够编译成功，但是不产生可执行文件
cargo check
# release
cargo build --release
# Updating crates.io index(Cargo.lock)
cargo update
```

Cargo主要两个profile
1. dev profile: `cargo build`
2. release profile: `cargo build --release`

自定义[profile](https://doc.rust-lang.org/cargo/reference/profiles.html)

```toml
[package]
name = "minigrep"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Guess Game

```toml
[package]
name = "hell_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
# 引入rand包
rand = "0.8.5" 
```

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude // trait

fn main() {
    println!("Guess!");
    // let foo=1; // immutable
    // let bar =foo;// immutable
    let secret_num = rand::thread_rng().gen_range(1..101); // [1, 101)
    println!("secret_num={}", secret_num);

    loop {
        let mut num = String::new(); // mutable
        io::stdin().read_line(&mut num).expect("cannot read line!");
        // shadow
        // let num:i32=num.trim().parse().expect("Please enter a integer");
        // 解析错误，程序退出
        // let num:u32=num.trim().parse().expect("Please enter a integer");
        // 解析错误，程序不退出
        let num: u32 = match num.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        println!("Your Num={}", num);

        match num.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Code management

模块系统(从上往下)
- Package: 用于 build, test, share crate
- Crate: 用于产生library, binary
- Module: 配合`use`使用，控制代码的组织、作用域、私有路径
- Path: 为struct, function, module命名的方式

Crate类型：
- binary
- library

Crate Root
- 是源代码文件，表示Rust编译器从这里开始，组成你的Crate的根Module

Package包含
- Cargo.toml, 描述如何构建这些Crates
- 包含0个或者1个 library crate
- 包含任意数量的binary crate
- 但是必须至少包含一个crate(binary or library)

```bash
$ cargo new project1
.
├── Cargo.toml
├── src
│   └── main.rs # 默认是binary crate的crate root，crate名于package名相同
```

如果有library

```bash
.
├── Cargo.toml
└── src
    ├── main.rs
    └── lib.rs # package包含一个libray crate, 作为library crate的crate root, crate名与package名相同
```

Cargo会把crate root文件交给rustc来build library or binary

一个Package可以有多个binary crate:
- 文件放在src/bin
- 每个文件都是单独的binary crate

Module:
- 在一个Crate内，将代码进行分组
- 控制项目的public, private
- 建立modulde使用`mod`
- 可以包含其他条目，比如sruct, enum, trait, function...，默认都是`private`
- 父级模块无法访问子模块的private
- 子模块可以使用所有祖先模块的所有条目

```rs
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

src/main.rs和src/lib.rs都叫做crate roots, 这两个文件的内容形成了名为crate的模块，违约整个模块树的根部

```bash
crate # crate root
└── front_of_house
    └── hostig
    │    ├── buildadd_to_waitlist
    │    └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └──take_payment
```

### Path

- 绝对路径: 从crate root开始，使用crate名或者字面值`crate`
- 相对路径：从当前路劲开始，self, super(上一级)或者当前模块的标识符
- 标识符以`::`分割

```rs
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    // 绝对路径调用, recommended
    // 虽然front_of_house没有使用pub修饰，但是因为都是同一个文件根级
    crate::front_of_house::hosting::add_to_waitlist();
    // crate::front_of_house::serving::serve_order(); // error, serving is private
    // 相对路径调用
    front_of_house::hosting::seat_at_table();
}
```

```rs
// src/lib.rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 相对调用
        super::serve_order();
        // 绝对调用
        crate::serve_order();
    }

    fn cook_order() {}
}
```

struct:
- `pub struct`将struct设置成public
- struct里面字段默认是private, 除了`pub struct`还得单独设置里面字段的可见性

```rs
// src/lib.rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String, // private
    }
}
```

```rs
// src/lib.rs
mod back_of_house {
    pub enum Appetizer {
        // enum前面添加pub，里面的字段都是pub
        Sourp,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        fruit: String, // private
    }

    impl Breakfast {
        pub fn summer(toast:&str) ->Breakfast{
            Breakfast { toast: String::from(toast), fruit: String::from("apple") },
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal=back_of_house::Breakfast::summer("Rye");
    meal.toast=String::from("Wheat");
    // meal.fruit=String::from("Peach");// error, private
}
```

### `use`

- 引入函数的时候，use指定到函数的父级

```rs
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}

        fn take_payment() {}
    }
}

// use AbsolutePath
use crate::front_of_house::hosting;
// 相当于 mod hosting{}

// // use RelativePath
// // 因为front_of_house是同一级，所以直接去掉crate就行
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // 引入函数的时候，use指定到函数的父级，比如hosting
    // hosting::seat_at_table(); // error, private
}
```

- 引入struct, enum的时候直接指定到struct, enum

```rs
// src/main.rs
use std::collections::HashMap;

fn main() {
    let mut map=HashMap::new();
    map.insert(1, 12.5);
}
```

- 同名的条目指定到父级

```rs
use std::fmt;
use std::io;

fn f1() -> fmt::Result {}

fn f2() -> io::Result {}

fn main() {}
```

- 使用`as`避免同名条目

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn f1() -> Result {}

fn f2() -> IoResult {}

fn main() {}
```

- `use`的模块默认是private，想要外部也能访问，使用`pub use`

`pub use crate::front_of_house::hosting;`

### Package

如果vscode的`rust-analyzer`一直卡住，先F1 `Stop`，然后`cargo build`

如果获取<https://crates.io/>速度太慢，编辑 `~/.cargo/config` 文件，添加以下内容：

```bash
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

标准库(std)也被当作是外部包
- 不需要修改Cargo.toml来包含std
- 需要使用特定条目的，才使用`use std::`


```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101); // [1, 101)
    println!("secret_num={}", secret_num);
}
```

- 简化同一个父级下的子条目

```rs
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101); // [1, 101)
    println!("secret_num={}", secret_num);
}
```

` 引入父级和子级

```rs
// use std::io;
// use std::io::Write;

// 相当于上面的情况
use std::io{self, Write};
```

- 引入某个条目下面所有的使用通配符 `use std::collections::*;`
  - 使用场景：将所有的被测试代码引入到tests模块

### module to different files

将模块拆分到多个文件

```bash
src
├── front_of_house.rs
├── lib.rs
└── main.rs
```

```rs
// main.rs
fn main() {}
```

```rs
// lib.rs
mod front_of_house; // ;意味着，将从front_of_house.rs里面找
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

```rs
// front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}
    fn serve_order() {}

    fn take_payment() {}
}
```

进一步拆分: 在模块后面加`;`，然后 将`{}`里面的内容移动到文件里面

```bash
src/
├── front_of_house
│   ├── hosting.rs
│   └── serving.rs
├── front_of_house.rs
├── lib.rs # 未改变
└── main.rs # 未改变
```

```rs
// front_of_house.rs
pub mod hosting;
mod serving;
```

```rs
// serving.rs
fn take_order() {}
fn serve_order() {}

fn take_payment() {}
```

```rs
// hosting.rs
pub fn add_to_waitlist() {}
fn seat_at_table() {}
```