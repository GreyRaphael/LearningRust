# Smart Pointer

- [Smart Pointer](#smart-pointer)
  - [introduction](#introduction)
  - [`Box<T>`](#boxt)
  - [Deref coercion](#deref-coercion)
  - [Drop Trait](#drop-trait)
  - [`Rc<T>`](#rct)
  - [`RefCell<T>` and Interior mutability](#refcellt-and-interior-mutability)
    - [`Rc<T>` with `RefCell<T>`](#rct-with-refcellt)
  - [Memory Leak](#memory-leak)
    - [`Weak<T>`](#weakt)
  - [Raw Pointer](#raw-pointer)

## introduction

> rust最常见的指针就是**引用**: `&`

智能指针: 比如`String`, `Vec<T>`
- 行为于指针类似, 拥有一片内存，并允许用户对其操作
- 有额外的元数据和功能，比如`capacity, length`

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

## Deref coercion

> 隐式解引用转化
- 当把某类型的引用传递给函数或方法的时候，它的类型与函数的参数类型不匹配，Deref Coercion就会发生
- 编译器对deref进行一系列调用，在编译时完成，没有额外的性能开销

```rs
fn main() {
    let n = 100;
    func(&n);
}

fn func(num: &i32) {
    println!("{:p}", num); // 0x7fffdf60b654
    println!("{}", num); // 100, 隐式解引用
    println!("{}", *num); // 100
    let b=num; // &i32
    let c=*num; // i32
}
```

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

fn hello(name: &str) {
    println!("hello, {}", name);
}

fn main() {
    let b1 = MyBox::new(String::from("Biden")); // MyBox<String>
    println!("{:?}", b1); // MyBox("Biden")
    println!("{}", *b1); // *b1 is String

    let s1 = String::from("Trump");
    hello("Grey"); // input is &str
    // hello(s1); // error, iput is String
    hello(&s1); // input is &String
    hello(&s1[..]); // input is &str

    // hello(*b1); // error
    hello(&(*b1)); // input is &String
    hello(&(*b1)[..]); // input is &str
    // &b1   &MyBox<String>
    // &b1作为参数传入，发生Deref coercion；调用deref, 将&MyBox<String>转化为&String
    hello(&b1); //ok, Deref coercion
}
```

## Drop Trait

- 常用于文件、网络资源的释放
- 任何类型都能实现Drop Trait
- Drop Trait只要求实现`drop`方法

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping, {}", self.data);
    }
}

fn main() {
    let c1 = CustomSmartPointer {
        data: String::from("Biden"),
    };
    let c2 = CustomSmartPointer {
        data: String::from("Trump"),
    };
    let c3 = CustomSmartPointer {
        data: String::from("John"),
    };
    drop(c3); // 库函数提前清理c3
    println!("Smart Pointer Created");
}
// // output:
// Dropping, John
// Smart Pointer Created
// Dropping, Trump
// Dropping, Biden
```

## `Rc<T>`

Reference Counted Smart Pointer, 引用计数智能指针:
> 只能用于单线程的场景
- 记录所有者的数量，使得一份数据被多个所有者持有
- 所有者数量为0的时候，自动清理数据

常用方法
- `Rc::clone(&a)`: 增加引用计数
- `Rc::strong_count(&a)`: 获取引用计数
- `Rc::weak_count(&a)`

example: 两个指针指向同一个List

```rs
// 错误示例
use std::rc::Rc;
use List::{Cons, Nil}; // 使用自定义的mod

fn main() {
    // 递归创建链表
    let li1 = Cons(11, Box::new(Cons(22, Box::new(Cons(33, Box::new(Nil))))));

    let li2 = Cons(100, Box::new(li1));
    // let li3 = Cons(100, Box::new(li1)); // error, li1 moved
}

enum List {
    Cons(i32, Box<List>),
    Nil, // 链表结尾
}
```

```rs
// 解决方案1：改变List为ref
use List::{Cons, Nil}; // 使用自定义的mod

fn main() {
    // 递归创建链表，需要保存临时值无法通过下面方法创建
    // 因为临时变量所有权被丢弃
    // let li1 = Box::new(Cons(22, &Box::new(Cons(33, &Box::new(Nil))))); // Box<List>

    let p1=Box::new(Nil);
    let c1=Cons(33, &p1);
    let p2=Box::new(c1);
    let c2=Cons(22, &p2);
    let p3=Box::new(c2);

    let li2 = Cons(100, &p3);
    let li3 = Cons(100, &p3); // error, li1 moved
}

enum List<'a> {
    Cons(i32, &'a Box<List<'a>>),
    Nil, // 链表结尾
}
```

> `Rc<T>`通过**不可变引用**，使得程序不同部分之间共享只读数据

```rs
// 解决方案2： Rc
use std::rc::Rc;
use List::{Cons, Nil}; // 使用自定义的mod

fn main() {
    // 递归创建链表
    let li1 = Rc::new(Cons(22, Rc::new(Cons(33, Rc::new(Nil))))); // Rc<List>
    // li1.clone(); // 深拷贝，不合适

    let li2 = Cons(100, Rc::clone(&li1)); // clone,引用计数+1，不进行深拷贝
    let li3 = Cons(100, Rc::clone(&li1));
}

enum List {
    Cons(i32, Rc<List>),
    Nil, // 链表结尾
}
```

`strong_count`

```rs
use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    // 递归创建链表
    let li1 = Rc::new(Cons(22, Rc::new(Cons(33, Rc::new(Nil))))); // Rc<List>
    println!("count={}", Rc::strong_count(&li1)); //1

    let li2 = Cons(100, Rc::clone(&li1));
    println!("count={}", Rc::strong_count(&li1)); //2
    {
        let li3 = Cons(100, Rc::clone(&li1));
        println!("count={}", Rc::strong_count(&li1)); //3
    }
    println!("count={}", Rc::strong_count(&li1)); //2
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
```

## `RefCell<T>` and Interior mutability

`RefCell<T>`: 代表了对齐持有数据的唯一所有权, 只能用于**单线程**场景

| `Box<T>`         | `RefCell<T>`   |
|----------------|--------------|
| 编译阶段强制代码遵守借用规则 | 只会在运行时检查借用规则 |
| 否则触发错误         | 否则触发panic    |

Rust借用规则
- 在任何给定时间，要么只拥有一个**可变应用**，要么拥有多个**不可变引用**
- 引用总是有效的

内部可变性(interior mutability): 允许在只持有**不可变引用**的情况下，对数据进行修改，是Rust的一种设计模式
> 数据结构中使用了`unsafe`代码，来绕过Rust正常的可变性和借用规则

| **编译阶段检查借用规则**             | **运行时检查借用规则**                   |
|------------------------|-----------------------------|
| 尽早暴露问题                 | 问题暴露延后，甚至到生产环境              |
| 没有任何运行时开销              | 因为借用计数产生些许性能损失              |
| 对大多数场景都是最佳选择(Rust默认行为) | 实现某些特定的内存安全场景(不可变环境中修改自身数据) |

选择`Box<T>`, `Rc<T>`, `RefCell<T>`依据
> 所以`RefCell<T>`可以实现不可变环境中修改自身数据

|     | `Box<T>`          | `Rc<T>`        | `RefCell<T>`      |
|----------|-----------------|--------------|-----------------|
| 同一数据的所有者 | 一个              | 多个           | 一个              |
| 可变性、借用检查 | 可变、不可变引用(编译时检查) | 不可变引用(编译时检查) | 可变、不可变引用(运行时检查) |

example: 无法可变借用(`&mut`)一个immutable

```rs
// lib.rs
fn main() {
    let x = 10;
    let y = &x; // ok
    // let z = &mut x; // error，无法可变借用(&mut)一个immutable
}
```

example: without `RefCell<T>`

```rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f64 / self.max as f64;
        if percent_of_max >= 1.0 {
            self.messenger.send("Error: you are over 100%");
        } else if percent_of_max >= 0.9 {
            self.messenger.send("Urgent: you are over 90%")
        } else if percent_of_max >= 0.75 {
            self.messenger.send("Warning: you are over 75%")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        send_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&mut self, msg: &str) { // error, Messenger是&self, immutable，但是这里需要&mut
            self.send_messages.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_over75_msg() {
        let mock_msger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msger, 100);
        limit_tracker.set_value(80);
        assert_eq!(1, mock_msger.send_messages.len());
    }
}
```

example: with `RefCell<T>`
> `RefCell<T>`会记录当前存在多少个活跃的`RefMut<T>`和`Ref<T>`; 
- 每次调用`borrow`，不可变借用计数+1, 离开作用域，不可变借用计数-1
- 每次调用`borrow_mut`，可变借用计数+1, 离开作用域，可变借用计数-1
- 以此技术来维护借用检查规则，任何时间只允许拥有多个不可变借用或者一个可变借用

```rs
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // borrow_mut获取内部值的可变引用，返回RefMut<T>
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_over75_msg() {
        let mock_msger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msger, 100);
        limit_tracker.set_value(80);
        // borrow获取内部值的不可变引用, 返回Ref<T>
        assert_eq!(1, mock_msger.send_messages.borrow().len());
    }
}
```

### `Rc<T>` with `RefCell<T>`

实现拥有多重所有权的可变数据

```rs
use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c); // c=Cons(RefCell { value: 10 }, Cons(RefCell { value: 5 }, Nil))
    *value.borrow_mut() += 10;
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c); // c=Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}
```

## Memory Leak

Rust的内存安全机制可以保证很难发生内存泄漏，但可以使用`Rc<T>`和`RefCell<T>`可以创造循环引用，导致内存泄漏

```rs
use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("count={}, next item={:?}", Rc::strong_count(&a), a.tail()); // count=1, next item=Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("count={}, next item={:?}", Rc::strong_count(&a), a.tail()); // count=2, next item=Some(RefCell { value: Nil })
    println!("count={}, next item={:?}", Rc::strong_count(&b), b.tail()); // count=1, next item=Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("count={}", Rc::strong_count(&a)); // count=2
    println!("count={}", Rc::strong_count(&b)); // count=2

    println!("a next time={:?}", a.tail()); // stack overflow
}
```

上面例子:` Rc::clone`为`Rc<T>`实例的strong_count加1, `Rc<T>`的实例只有在strong_count为0的时候才会被清理
> 因为strong_count=2，所以出现死循环

solution1:

```rs
use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=0, next item=Some(RefCell { value: (Weak) })

    let b = Rc::new(Cons(10, RefCell::new(Rc::downgrade(&a))));
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=1, next item=Some(RefCell { value: (Weak) })
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b),
        b.tail()
    ); // strong count=1, weak count=0, next item=Some(RefCell { value: (Weak) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=1, next item=Some(RefCell { value: (Weak) })
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b),
        b.tail()
    ); // strong count=1, weak count=1, next item=Some(RefCell { value: (Weak) })
}
```

solution2:

```rs
use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<Rc<List>> {
        match self {
            Cons(_, item) => item.borrow().upgrade(),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=0, next item=None

    // downgrade: Rc<List> -> Weak<List>
    let b = Rc::new(Cons(10, RefCell::new(Rc::downgrade(&a))));
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=1, next item=None
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b),
        b.tail()
    ); //  strong count=1, weak count=0, next item=Some(Cons(5, RefCell { value: (Weak) }))

    // *a is Cons(i32, RefCell<Weak<List>>)
    // &*a is Cons(i32, &RefCell<Weak<List>>)
    if let Cons(_, item) = &*a {
        *item.borrow_mut() = Rc::downgrade(&b);
    }

    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
        a.tail()
    ); // strong count=1, weak count=1, next item=Some(Cons(10, RefCell { value: (Weak) }))
    println!(
        "strong count={}, weak count={}, next item={:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b),
        b.tail()
    ); //  strong count=1, weak count=1, next item=Some(Cons(5, RefCell { value: (Weak) }))
}
```

### `Weak<T>`

`Rc<T>`实例通过调用`Rc::downgrade`方法可以创建值的Weak Reference(弱引用)
- 返回类型是`Weak<T>`，为智能指针
- 调用`Rc::downgrade`会为weak_count加1
- weak_count不为0， 不影响`Rc<T>`实例的清理，可以避免循环引用

```rs
use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    // tree
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // upgrade, Weak<T> -> Rc<T>
    println!("parent={:?}", leaf.parent.borrow().upgrade()); // parent=None
    println!(
        "leaf strong count={}, weak count={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // downgrade: Rc<T> -> Weak<T>
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong count={}, weak count={}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        ); // branch strong count=1, weak count=1;
        println!(
            "leaf strong count={}, weak count={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    } // 对于branch,走出作用域，强引用-1，虽然弱引用为1，不妨碍丢弃branch

    println!("parent={:?}", leaf.parent.borrow().upgrade()); // parent=None
    println!(
        "leaf strong count={}, weak count={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
```

## Raw Pointer

example: pointer size

```rs
use std::mem::size_of;

static B: [u8; 10] = [99,97,114,114,121,116,111,119,101,108];
static C: [u8; 11] = [116,104,97,110,107,115,102,105,115,104,0];

fn main() {
    let a = 42;
    let b1 = &B;
    let b2 = Box::new(B); // smart pointer
    let c = &C;

    println!("a : addr={:p},size={} bytes, value={}", &a, size_of::<i32>(), a);
    println!("b1: addr={:p},size={} bytes, value={:p}", &b1, size_of::<&[u8;10]>(), b1);
    println!("b2: addr={:p},size={} bytes, value={:p}", &b2, size_of::<Box<[u8]>>(), b2);
    println!("c : addr={:p},size={} bytes, value={:p}", &c, size_of::<&[u8;11]>(), c);
    println!("B : addr={:p},size={} bytes, value={:?}", &B, size_of::<[u8;10]>(), B);
    println!("C : addr={:p},size={} bytes, value={:?}", &C, size_of::<[u8;11]>(), C);
}
```

```bash
# output, b2只能指向heap，所以不是static静态区的地址
a : addr=0x7fffa568825c,size=4 bytes, value=42
b1: addr=0x7fffa5688260,size=8 bytes, value=0x55b3c256504d
b2: addr=0x7fffa5688268,size=16 bytes, value=0x55b3c2818ad0
c : addr=0x7fffa5688280,size=8 bytes, value=0x55b3c2565057
B : addr=0x55b3c256504d,size=10 bytes, value=[99, 97, 114, 114, 121, 116, 111, 119, 101, 108]
C : addr=0x55b3c2565057,size=11 bytes, value=[116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0]
```

> Raw pointer is `unsafe`
- 不可变raw pointer: `*const T`. e.g. `*const u8`, `*cosnt String`
- 可变rawo pointer: `*mut T`
- 两者可以相互转换
- Rust编译过程会将`&T`和`&mut T`转换成raw pointer
- Raw pointer不拥有值得所有权，即，编译器不会检查合法性
- 允许多个Raw pointer指向同一个数据，编译器无法保证共享数据得合法性

```rs
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr:usize = unsafe { std::mem::transmute(a_ptr) };
    // value=42, ptr=0x7ffe15cb0a48, 0x7ffe15cb0a48, addr=7ffe15cb0a4f
    println!("value={}, ptr={:p}, {:p}, addr={:x}", a, &a, a_ptr, a_addr+7);
}
```

```rs
use std::borrow::Cow;
use std::ffi::CStr; // 类似C语言的字符串，以0结尾
use std::os::raw::c_char; // i8类型的alias

static B: [u8; 10] = [99,97,114,114,121,116,111,119,101,108];
static C: [u8; 11] = [116,104,97,110,107,115,102,105,115,104,0];

fn main() {
    let a = 42;
    let b:String;
    let c:Cow<str>;

    unsafe{
        let b_ptr=&B as *const u8 as *mut u8;
        b=String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr=&C as *const u8 as *const c_char; // *const i8
        c=CStr::from_ptr(c_ptr).to_string_lossy();
    }

    // a=42,b=carrytowel,c=thanksfish
    println!("a={},b={},c={}", a, b, c);
}
```

| 名称          | 简介| 强项 | 弱项 |
|-----|-----|-----|-----|
|`Raw Pointer` | `*const T`,`*mut T`| 速度、与外界交互 | unsafe|
|`Box<T>`      | 最常用的智能指针| 将值集中储存在Heap | 大小增加  |
|`Rc<T>`       | 值的共享访问 | 对值的共享访问 | 大小增加；运行时成本；线程不安全   |
|`Arc<T>`      | 可以跨线程共享值  | 对值的共享访问；线程安全 | 大小增加；运行时成本；|
|`Cell<T> `    | 具有改变不可变值的能力 | 内部可变性  | 大小增加；性能    |
|`RefCell<T>`  | 对不可变引用执行改变，但有代价    | 内部可变性；可与仅接受不可变引用的Rc,Arc嵌套使用 | 大小增加；运行时成本；缺乏编译时保障 |
|`Cow<T>`      | Clone on Write,封闭并提供对借用数据的不可变访问，并在需要修改或所有权的时候，延迟克隆数据 | 只读访问避免写入| 大小可能增加|
|`String`      | 可处理可变长度的文本 | 动态按需增长；保证正确编码  | 过度分配内存大小   |
|`Vec<T>`      | 程序最常用的存储系统；创建和销毁值时保持数据有序 | 动态按需增长 | 过度分配内存大小   |
|`RawVec<T>`   | 是`Vec<T>`和其他动态大小类型的基石 | 动态按需增长 | 不直接使用 |
|`Unique<T>`   | 作为值的唯一使用者，保证拥有完全的控制权 | 需要独占值的类型 | 不直接使用 |
|`Shared<T>`   | 分享所有权 | 共享所有权，可以将内存与T的宽度对齐  | 不直接使用 |