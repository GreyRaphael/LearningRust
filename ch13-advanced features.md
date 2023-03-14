# Advanced Features

- [Advanced Features](#advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [raw pointer](#raw-pointer)

## Unsafe Rust

Unsafe Rust存在原因
- 静态分析是保守的
- 计算机硬件本事是unsafe, Rust进行底层系统编程需要unsafe

Unsafe Rust四个功能
- 解引用原始指针(raw pointer)
- 调用unsafe函数或方法
- 访问或修改可变的静态变量
- 实现unsafe trait

### raw pointer

目的: 
- 与C语言进行接口
- 构建借用检查器无法理解的抽象

```rs
fn main(){
    let mut num=10;

    let raw_pointer1=&num as *const i32;
    let raw_pointer2=&mut num as *mut i32;
    
    unsafe{
        *raw_pointer2=2000;
        println!("value1={}", *raw_pointer1); // value1=2000
        println!("value2={}", *raw_pointer2); // value2=2000
    }

    let address=0x012345usize;
    let raw_pointer3= address as *const i32;
    unsafe {
        println!("value3={}", *raw_pointer3); // Segmentation fault (core dumped), illegal visit
    }
}
```