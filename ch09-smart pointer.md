# Smart Pointer

- [Smart Pointer](#smart-pointer)
  - [introduction](#introduction)

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

