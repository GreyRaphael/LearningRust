# LearningRust

> Rust Learning Notes

- [LearningRust](#learningrust)
  - [Preparation](#preparation)
    - [First Example](#first-example)
    - [Cargo](#cargo)
  - [Guess Game](#guess-game)
  - [Code management](#code-management)

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
- 可以包含其他子项，比如sruct, enum, trait, function...