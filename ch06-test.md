# Rust test

- [Rust test](#rust-test)
  - [`assert`](#assert)
  - [`assert_eq!`, `assert_ne!`](#assert_eq-assert_ne)

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