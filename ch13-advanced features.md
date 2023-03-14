# Advanced Features

- [Advanced Features](#advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [raw pointer](#raw-pointer)
    - [unsafe function](#unsafe-function)

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

### unsafe function

```rs
fn main(){
    unsafe{
        dangerous();
    }
}

unsafe fn dangerous(){}
```

```rs
use std::slice;

// override split_at_mut
// 具有不安全代码的安全抽象
fn split_at_mut(a_slice: &mut[i32], mid: usize)->(&mut[i32], &mut[i32]){
    let length=a_slice.len();
    let raw_ptr=a_slice.as_mut_ptr();// *mut i32

    assert!(mid<=length);
    unsafe{
        (
            slice::from_raw_parts_mut(raw_ptr, mid),
            // .add(mid)以mid为偏移量的指针
            slice::from_raw_parts_mut(raw_ptr.add(mid), length-mid),
        )
    }
}

fn main(){
    let mut v=vec![1,2, 3, 4, 5, 6];
    let slice1=&mut v[..];
    let (a, b)=slice1.split_at_mut(3);
    assert_eq!(a, &mut[1, 2, 3]);
    assert_eq!(b, &mut[4, 5, 6]);
    println!("a={:?}, b={:?}", a, b); // a=[1, 2, 3], b=[4, 5, 6]

    // error example
    let addr=0x012345usize;
    let raw_ptr=addr as *mut i32;
    let slice2:&[i32]=unsafe {
        // slice::from_raw_parts_mut(raw_ptr, 10); // error
    };
}
```