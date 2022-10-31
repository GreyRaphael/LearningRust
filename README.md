# LearningRust

> Rust Learning Notes

- [LearningRust](#learningrust)
  - [Preparation](#preparation)
    - [First Example](#first-example)
    - [Cargo](#cargo)
  - [Guess Game](#guess-game)

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