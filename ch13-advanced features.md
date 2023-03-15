# Advanced Features

- [Advanced Features](#advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [raw pointer](#raw-pointer)
    - [unsafe function](#unsafe-function)
    - [`extern`](#extern)
    - [global variable, `static`](#global-variable-static)
    - [unsafe trait](#unsafe-trait)
  - [Advanced Trait](#advanced-trait)
    - [Associated Types](#associated-types)

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

### `extern`

作用：简化创建和使用外部函数接口(FFI, Foreign Function Interface)的过程
> 允许一种编程语言定义函数，并让其他编程语言调用这些函数  
> `extern`声明的函数都是`unsafe`

调用其他语言提供的函数

```rs
extern "C" { // 遵循C语言的ABI
    fn abs(input :i32)->i32;
}

fn main() {
    unsafe{
        println!("abs = {}", abs(-3));
    }
}
```

暴露rust函数给其他语言使用

```rs
#[no_mangle] // 不允许编译器改函数的名字
pub extern "C" fn call_from_c(){
    println!("Just call a function from C!");
}

fn main(){}
```

### global variable, `static`

Rust支持全局变量，但因为所有权机制可能产生数据竞争问题

```rs
// 生命周期默认都是'static, 无需显式标注
static G_NUM: &str="Biden";
static mut COUNTER: u32=0;

fn add_to_count(inc:u32){
    unsafe{
        COUNTER+=inc;
    }
}

fn main(){
    println!("name is: {}", G_NUM);

    add_to_count(3);
    unsafe{
        // 并行或者并发的时候不要使用静态变量
        println!("{}", COUNTER); // 3
    }
}
```

### unsafe trait

```rs
unsafe trait Foo{

}

unsafe impl Foo for i32{

}

fn main(){}
```

## Advanced Trait

### Associated Types

关联类型(Associated Types) vs 泛型
- 关联类型实现需要`type Item=u32`，而且只能针对`Conter`实现一种类型的trait(u32)
- 泛型实现需要标注`<u32>, <Strng>`，能够针对`Conter`实现多种种类型的trait(u32, String)

```rs
pub trait Iterator {
    type Item; // Item is Associated Types

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    // 泛型
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32; // 实现的过程需要标注 u32, 而且只能实现一次
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
}

fn main() {}
```

