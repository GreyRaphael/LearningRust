# Rust test

- [Rust test](#rust-test)
  - [`assert`](#assert)
  - [`assert_eq!`, `assert_ne!`](#assert_eq-assert_ne)
  - [custom information](#custom-information)
  - [`should_panic`](#should_panic)
  - [`Result<T, E>`](#resultt-e)
  - [`cargo test` parameter](#cargo-test-parameter)
  - [unit test \& integration test](#unit-test--integration-test)

3A操作
- Arrange: 准备数据、状态
- Act: 运行被测试的代码
- Assert: 断言结果

具体过程
- Arrange: 测试函数需要`test`属性进行标注: 在函数上面加上`#[test]`，即可变成测试函数
- Act: `cargo test`
- Assert: 

当使用cargo创建library项项目的时候，会生成一个test module，里面有一个test函数。也可以添加任意数量的test module或函数
> `cargo new project_name --lib`

```rs
// lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

如果触发`panic`就代表测试失败

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn another1() {
        panic!("Make this test fail");
    }
    #[test]
    fn another2() {
        let result = 2 + 2;
        assert_eq!(result, 5);
    }
}
```

## `assert`

`assert!`宏，来自标准库，用来确定某个状态是否为true
- true: 测试通过
- false: 调用`panic!`，测试失败

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 1,
            width: 8,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

## `assert_eq!`, `assert_ne!`

判断两个参数是否`==` or `!=`
> 断言失败，使用debug模式打印参数，要求参数实现了PartialEq和Debug Traits(所有的基本类型都实现了，自定义类型需要自己实现)  

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 1,
            width: 8,
        };
        // assert!(larger.can_hold(&smaller));
        assert_eq!(true, larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        // assert!(!smaller.can_hold(&larger));
        // assert_eq!(false, smaller.can_hold(&larger));
        assert_ne!(true, smaller.can_hold(&larger)); // 两个参数顺序可以调换，一般将期待值放在左边
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

## custom information

- `assert!`宏的第2个参数就是自定义信息
- `assert_eq!`宏的第3个参数就是自定义信息
- `assert_ne!`宏的第3个参数就是自定义信息

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_contain_name() {
        let result = greeting("moris");
        assert!(result.contains("grey"), "custom info: value={}", result);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}
```

## `should_panic`

除了验证返回值是否正确，还需验证是否发生panic, 可以添加`should_panic`属性
- 发生panic，测试通过
- 未发生panic，测试不通过

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be [1, 100]");
        }
        Guess { value }
    }
}
```

让`should_panic`更加仔细, 只期待一部分

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "value should be >0")]
    fn less_than_1() {
        // Guess::new(0); // pass
        Guess::new(200); // fail
    }

    #[test]
    #[should_panic(expected = "value should be <101")]
    fn greater_than_100() {
        // Guess::new(200);//pass
        Guess::new(0); //fail
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("value should be >0, got {}", value)
        } else if value > 100 {
            panic!("value should be <101, got {}", value)
        }
        Guess { value }
    }
}
```

## `Result<T, E>`

无需panic,使用`Result<T, E>`作为返回类型来测试
- 返回Ok，pass
- 返回Err, fail

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("some error information"))
        }
    }
}
```

## `cargo test` parameter

通过命令行参数改变`cargo test`的行为

默认行为：
- 并行运行所有测试
- 测试通过不显示所有输出内容，测试失败才输出大量内容

cmd
- `cargo test --help`: 获取所有帮助
- `cargo test -- --help`: 可以使用`--`的参数，有一个编译的过程

并行测试：默认状态
- 多个线程并行运行
- 要确保多个测试之间，不会相互依赖
- 要确保多个测试，不依赖某个共享的状态(环境、工作目录、环境变量....)

串行测试: `cargo test -- --test-threads=1`

测试代码中用了`println!`, 如果要全部显示`cargo test -- --show-output`
- 测试pass, 不打印`println!`的内容
- 测试fail, 打印`println!`的内容

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_pass() {
        assert_eq!(10, print_and_return_10(4));
    }
    #[test]
    fn will_fail() {
        assert_eq!(200, print_and_return_10(8));
    }
}

fn print_and_return_10(a: i32) -> i32 {
    println!("I got a value={}", a);
    10
}
```

控制测试函数的数量:
- `cargo test will_pass`: 只测试`will_pass`函数
- `cargo test will`: 测试`will`开头的所有函数
- `cargo test tests`: 测试`mod tests`的所有函数

忽略某些测试`#[ignore]`, `cargo test`就不允许ignored的测试
> 如果要单独测试`ignore`部分，`cargo test -- --ignored`

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
    #[test]
    #[ignore = "time cost too much"]
    // #[ignore] // 也可以不带自定义信息
    fn expensive_test() {
        assert_eq!(8, 2 + 2 + 2 + 2);
    }
}
```

## unit test & integration test

Rust测试分类
- unit test: `#[cfg(test)]`
  - 小、专注
  - 一次对一个模块进行隔离的测试
  - 可测试private接口或者函数: 比如上文的测试函数有的有`pub`有的没有`pub`
- integration test: 为了测试被测试库的多个部分是否能够一起正常工作
  - 集成测试完全位于被测试库的外部
  - 只能使用public接口，所以集成测试覆盖率很重要
  - 每个测试中可能使用多个模块

集成测试步骤
1. 创建目录`tests`， 该目录被特殊对待
2. tests目录下每个测试文件都是一个单独的crate, 这些文件不共享行为(与src目录不同)
3. 测试需要导入测试库lib.rs中的东西
4. 标注`#[test]`
5. 只有`cargo test`才会编译tests目录下的文件
   1. 运行某一个特定集成测试: `cargo test function_name`
      - e.g. `cargo test it_adds_three`
   2. 运行某个测试文件内所有的测试: `cargo test --test filename`
      - e.g. `cargo test --test another_integration_test`


```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
    └── another_integration_test.rs
```

```toml
# Cargo.toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
```

```rs
// lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

```rs
// integration_test.rs
use adder;

// integration test
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

```rs
// another_integration_test.rs
use adder;

// integration test
#[test]
fn it_adds_three() {
    assert_eq!(5, adder::add_two(3));
}
```

example: 集成测试里面的非测试的用于帮助的模块放到tests的子目录

```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── another_integration_test.rs
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

```rs
// mod.rs
pub fn test_helper() {
    println!("this is a helper");
}
```

```rs
// integraton_tsts.rs
use adder;
mod common;

// integration test
#[test]
fn it_adds_two() {
    common::test_helper();
    assert_eq!(6, adder::add_two(2));
}
```