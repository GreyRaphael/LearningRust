# Rust test

- [Rust test](#rust-test)
  - [`assert`](#assert)
  - [`assert_eq!`, `assert_ne!`](#assert_eq-assert_ne)
  - [custom information](#custom-information)
  - [`should_panic`](#should_panic)

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