# Lifetime

- [Lifetime](#lifetime)
  - [dangling reference](#dangling-reference)
  - [lifetime specifier](#lifetime-specifier)
  - [lifetime with struct](#lifetime-with-struct)
  - [lifetime rules](#lifetime-rules)

生命周期: 让**引用**保持有效的作用域
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