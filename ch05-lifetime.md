# Lifetime

- [Lifetime](#lifetime)
  - [dangling reference](#dangling-reference)
  - [lifetime specifier](#lifetime-specifier)
  - [lifetime with struct](#lifetime-with-struct)
  - [lifetime rules](#lifetime-rules)
    - [`'static` lifetime](#static-lifetime)
  - [lifetime bound](#lifetime-bound)
  - [lifetime with generics](#lifetime-with-generics)
  - [advanced](#advanced)
    - [`&'static` vs `T:'static`](#static-vs-tstatic)

> **生命周期标注并不会改变任何引用的实际作用域**

生命周期: **引用**保持有效的作用域
> 生命周期目的是为了避免`dangling reference`
- rust每个**引用**都有生命周期
- 大多数生命周期都是隐式的，可被推断的
- 当引用的生命周期可能以不同的方式互相关联时，需要手动标注生命周期

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 因为所有参数都是ref, 所有需要指定lifetime相同，使用'a手动标记生命周期
    // https://stackoverflow.com/questions/59522097/rust-lifetime-mismatch-in-trait-method
    fn mixup<'a, V, W>(&'a self, other: &'a Point<V, W>) -> Point<&T, &W> {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // <&str, char>
    let p3 = p1.mixup(&p2);
    println!("{}-{}", p1.x, p1.y); // 10-20.2
    println!("{}-{}", p2.x, p2.y); // hello-c
    println!("{}-{}", p3.x, p3.y); // 10-c
}
```

## dangling reference

> rust编译器采用**借用检查器**：比较作用域来判断所有的借用是否合法


example: r的生命周期包含x

```rs
fn main() {
    {
        let r;
        {
            let x = 5;
            // r = &x; // error, 因为x离开作用域之后，被清理了，x变成了dangline ref
        }
        println!("r:{}", r);
    }
}
```

```rs
// 正确做法，x的生命周期包含了r的生命周期
fn main() {
    let x = 5;
    let r = &x;
}
```

example: 比较字符串长短

```rs
fn main() {
    let s1 = String::from("Trump");
    let s2 = "Biden";
    let result = longest(s1.as_str(), s2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 因为所有参数都是ref, 返回值无法判断生命周期到底跟x还是y
    // 所以标记出生命周期，对齐
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## lifetime specifier

> 生命周期标注：描述了多个引用的生命周期间的关系，但不影响生命周期

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 函数longest的生命周期是'a，本质是x,y生命周期重叠的部分，即x和y生命周期短的那一个
    // longest的生命周期不小于x的生命周期
    // longest的生命周期不小于y的生命周期
    // 返回值的生命周期不小于'a标注的什么周期
    // 所以result能够存活到println!()
}
```

```rs
fn longest<'a>(x: &'a str, y: & str) -> &'a str {
    // 如果返回值和y无关，可以不用配置y的生命周期
    x
}
```

```rs
// error example
fn main() {
    let s1 = String::from("Trump");
    {
        let s2 = "Biden"; // s2是字面值，生命周期和程序一样，但是作用域无法被下面看见
    }
    let result = longest(s1.as_str(), s2); // cannot find s2
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rs
// error example
fn main() {
    let s1 = String::from("Trump");
    let result;
    {
        let s2 = String::from("Biden");
        result = longest(s1.as_str(), s2.as_str()); // error, `s2` does not live long enough
    }
    println!("{}", result); // error, longes生命周期在花括号里面，所以result的生命周期也在花括号里面
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

tips:
- 从函数返回引用时，返回类型的生命周期参数需要和其中一个参数的生命周期匹配；
- 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值，该值在函数结束时就走出了Scope, 返回值就变成了dangling reference

```rs
// error example
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result=String::from("Putin");
    // result.as_str() // error, dangling reference
}
```

```rs
// correct example
fn longest<'a>(x: &'a str, y: &'a str) -> String {
    let result=String::from("Putin");
    result // 直接返回值，而不是ref, 转移所有权到函数外面
}
```

## lifetime with struct

```rs
fn main() {
    let novel = String::from("Hello Trump. Hello Biden");
    let first_setence = novel.split('.').next().expect("cannot find .");// &str
    let i = ImportantExcerpt {
        part: first_setence,
    }; //因为first_setence的生命周期长于实例i，所以可行
}

struct ImportantExcerpt<'a> {
    part: &'a str, // part的lifetime长于结构体实例
}
```

## lifetime rules

编译器使用3个规则，在没有显式标注生命周期的情况下，来确定生命周期
1. 每个引用类型的参数都有自己的生命周期。如果输入参数是2个，那么就有2个生命周期
2. 如果只有1个输入生命周期参数，那么该生命周期被赋予所有的输出生命周期参数
3. 如果有多个输入生命周期参数，但其中一个是`&self`或者`&mut self`，那么self的生命周期会赋予所有的输出生命周期, 仅仅针对该struct的方法

- 规则1应用于输入生命周期
- 规则2，3应用于输出生命周期
- 如果编译器完成3个规则的检查，仍然无法确定生命周期，那么就报错
- 这些规则适用于`fn`和`impl`块

```rs
fn main() {
    let novel = String::from("Hello Trump. Hello Biden");
    let first_setence = novel.split('.').next().expect("cannot find ."); // &str
    let i = ImportantExcerpt {
        part: first_setence,
    };
    i.announce_and_return_part("hello");
}

struct ImportantExcerpt<'a> {
    part: &'a str, // part的lifetime长于结构体实例
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        123466
    }

    // 因为不是泛型，并且有self, 那么自动将self的生命周期赋予给输出生命周期
    fn announce_and_return_part(&self, announcement:&str)->&str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

前文泛型的生命周期解释

```rs
struct Point {
    x: i32,
    y: f64,
}

struct Rectangle<'a> {
    width: &'a i32,
    height: &'a f64,
}

impl Point {
    fn mixup<'a>(&'a self, other: &'a Point) -> Rectangle {
        // 不同的struct，所以规则3不适用
        Rectangle {
            width: &self.x,
            height: &other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: 100, y: 120.2 }; // <i32, f64>
    let p3 = p1.mixup(&p2);
    println!("{}-{}", p1.x, p1.y); // 10-20.2
    println!("{}-{}", p2.x, p2.y); // hello-c
    println!("{}-{}", p3.width, p3.height); // 10-c
}
```

### `'static` lifetime

静态生命周期，整个程序的持续时间
> 比如字符串字面值, `let s1:&'static str="hello world";`

## lifetime bound

```rs
fn main() {
    let novel = String::from("Hello Trump. Hello Biden");
    let first_setence = novel.split('.').next().expect("cannot find ."); // &str
    let i = ImportantExcerpt {
        part: first_setence,
    };
    i.announce_and_return_part("hello");
}

struct ImportantExcerpt<'a> {
    part: &'a str, // part的lifetime长于结构体实例
}

impl<'a> ImportantExcerpt<'a> {
    // 修改返回值的生命周期为'b，但是实际上其生命周期应该是'a
    // 所以self.part要比返回值获得时间就，'a>'b
    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

因为`'a: 'b`表示`'a`生命周期大于`'b`，没有其他符号表示出`'a`生命周期小于`'b`, 而`<>`括号只表示声明，所以需要提前
> 上一个例子看起来更加直白明显

```rs
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## lifetime with generics

生命周期也是泛型的一种，所以他们放到同一个`<>`里面

```rs
use std::fmt::Display;

fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announce, {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let res = longest_with_announcement("Donarld Trump", "Biden", "info");
    println!("{}", res);// Donarld Trump
    let res = longest_with_announcement("Donarld Trump", "Biden", 100);
    println!("{}", res);// Donarld Trump
}
```

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<'a, 'b: 'a, V, W>(&'a self, other: &'b Point<V, W>) -> Point<&T, &W> {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // <&str, char>
    let p3 = p1.mixup(&p2);
    println!("{}-{}", p1.x, p1.y); // 10-20.2
    println!("{}-{}", p2.x, p2.y); // hello-c
    println!("{}-{}", p3.x, p3.y); // 10-c
}
```

```rs
impl<T, U> Point<T, U> {
    fn mixup<'a, 'b, V, W>(&'a self, other: &'b Point<V, W>) -> Point<&T, &W>
    where
        'b: 'a,
    {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
}
```

```rs
impl<'a, T, U> Point<T, U> {
    fn mixup<'b: 'a, V, W>(&'a self, other: &'b Point<V, W>) -> Point<&T, &W> {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
}
```

## advanced

### `&'static` vs `T:'static`

>`'static` 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 `'static`, 其中 `T:'static` 更为常用  
> 字面值都是`'static`数据，和程序活得一样久

simple compare

```rs
use std::fmt::Display;

fn main() {
    let mark_twain = "Samuel Clemens";
    print_author1(mark_twain);
    print_author2(mark_twain);
    // print_author2(&mark_twain); // error, &&str is not static
    print_author3(mark_twain);
    print_author3(&mark_twain);
    // print_author4(mark_twain); // error, str is not static
    print_author4(&mark_twain);
}

fn print_author1(author: &'static str) {
    println!(
        "print_author1 passed-in 'static value is: {}, of type &str",
        author
    );
}

fn print_author2<T: Display + 'static>(message: T) {
    println!(
        "print_author2 passed-in 'static value is: {}, of type {:?}",
        message,
        std::any::type_name::<T>()
    );
}

fn print_author3<T: Display>(message: T) {
    println!(
        "print_author3 passed-in 'static value is: {}, of type {:?}",
        message,
        std::any::type_name::<T>()
    );
}

fn print_author4<T: Display + 'static>(message: &T) {
    println!(
        "print_author4 passed-in 'static value is: {}, of type {:?}",
        message,
        std::any::type_name::<T>()
    );
}

// print_author1 passed-in 'static value is: Samuel Clemens, of type &str
// print_author2 passed-in 'static value is: Samuel Clemens, of type "&str"
// print_author3 passed-in 'static value is: Samuel Clemens, of type "&str"
// print_author3 passed-in 'static value is: Samuel Clemens, of type "&&str"
// print_author4 passed-in 'static value is: Samuel Clemens, of type "&str"
```

引用`'static`数据的变量遵循作用域范围，但是`'static`数据本身活得跟程序一样久; 如果出了变量的作用域而访问`'static`数据，就需要使用`unsafe`

```rs
fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 超出作用域时，该引用不能再被使用，但是数据依然会存在于 binary 所占用的内存中
    }

    println!("static_string reference remains alive: {}", static_string); // error
}
```

```rs
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn main() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
}
// The 12 bytes at 0x5571FE8BC000 stored: Hello World!
```

```rs
use std::fmt::Display;

fn main() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // -> 42
    println!("&'static str: {}", r2); // -> &'static str

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        static_bound(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        r3 = &s1;

        // s1 在这里被 drop
    }
    println!("{}", r3);
}

fn static_bound<T: Display + 'static>(t: &T) {
    println!("{}", t);
}
```

simple example for `T: 'static`

```rs
use std::fmt::Debug;

fn print_it11<T: Debug + 'static>(input: T) {
    println!(
        "'static value passed in is: {:?}, of type {:?}",
        input,
        std::any::type_name::<T>()
    );
}

fn print_it12(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it21<T: Debug + 'static>(input: &T) {
    println!(
        "'static value passed in is: {:?}, of type {:?}",
        input,
        std::any::type_name::<T>()
    );
}

fn print_it22(input: &(impl Debug + 'static)) {
    println!(
        "'static value passed in is: {:?}, of type {:?}",
        input,
        std::any::type_name::<dyn Debug>()
    );
}

fn main() {
    let i = 5;
    // let i = "hello";

    print_it11(i); // i这里直接使用的literal,所以是static
    print_it12(i); // i这里直接使用的literal,所以是static
    // print_it11(&i); // error
    // print_it12(&i);  // error
    print_it21(&i);// 约束的是T,但是使用的是&T,编译器不检查T
    print_it22(&i);// 约束的是T,但是使用的是&T,编译器不检查T
}
```