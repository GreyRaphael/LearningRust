# Smart Pointer

- [Smart Pointer](#smart-pointer)
  - [introduction](#introduction)
  - [`Box<T>`](#boxt)

## introduction

> rust最常见的指针就是**引用**: `&`

智能指针: 比如`String`, `Vec<T>`
- 行为于指针类似, 拥有一片内存，并允许用户对齐操作
- 有额外的元数据和功能，比如`capacity, length`

reference counting智能指针:
- 记录所有者的数量，使得一份数据被多个所有者持有
- 所有者数量为0的时候，自动清理数据

引用 vs 智能指针
- 引用只是借用
- 智能指针很多时候都拥有它所指向的数据

智能指针实现使用了struct
- 实现Deref trait, 允许智能指针struct能够像**引用**一样使用
- 实现Drop trait, 允许自定义当智能指针离开作用域的代码

标准库常见指针:
- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

> 内部可变模式(**interior mutability pattern**), 不可变类型暴露出可修改内部值的API


> 引用循环(**reference cycles**): 它如何泄露内存，以及如何防止发生

## `Box<T>`

> `Box<T>`是拥有一个元素的 **tuple struct**

```rs
fn main() {
    let b1 = Box::new(10); // Box<i32>
    println!("{}", b1); // 10
    println!("{}", *b1); // 10

    let x = 20;
    let y = &x;
    println!("{}", 20 == *y); // true

    let b2 = Box::new(x);
    println!("{}", 20 == *b2); // true
}
```

example: 创建一个链表，链表的节点为Cons, Cons的第一个元素是i32, 第二个元素是指针指向下一个元素

> Rust需要知道每一个类型占用的空间，但是递归类型比如下面的`List`大小是无法计算的，需要将其进行Box改造，因为Box是指针，大小能够确定

```rs
enum List {
    Cons(i32, List),
    Nil, // 链表结尾
}
```

```rs
use List::{Cons, Nil}; // 使用自定义的mod

fn main() {
    // // 递归创建链表
    // let li1 = Cons(11, Box::new(Cons(22, Box::new(Cons(33, Box::new(Nil))))));
    let p1 = Box::new(Nil);
    let c1 = Cons(33, p1);
    let p2 = Box::new(c1);
    let c2 = Cons(22, p2);
    let p3 = Box::new(c2);
    let c3 = Cons(11, p3);
}

enum List {
    Cons(i32, Box<List>),
    Nil, // 链表结尾
}
```

example: custom Box

```rs
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b1 = Box::new(10); // Box<i32>
    println!("{}", b1); // 10
    println!("{}", *b1); // 10

    let b3 = MyBox::new(100);
    println!("{:?}", b3); // MyBox(100)
    println!("{}", *b3); // 100, 相当于 *(b3.deref())
}
```