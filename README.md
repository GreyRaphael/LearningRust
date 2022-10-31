# LearningRust

> Rust Learning Notes

- [LearningRust](#learningrust)
  - [Preparation](#preparation)
    - [First Example](#first-example)
    - [Cargo](#cargo)

## Preparation

```bash
sudo apt update
sudo apt install rustc
sudo apt install cargo
sudo apt install rust-src

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
```