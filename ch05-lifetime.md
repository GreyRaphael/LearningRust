# Lifetime

- [Lifetime](#lifetime)
  - [dangling reference](#dangling-reference)

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