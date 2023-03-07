# Generic Data Type

- [Generic Data Type](#generic-data-type)
  - [function with generic](#function-with-generic)
  - [struct with generic](#struct-with-generic)
    - [enum with generic](#enum-with-generic)
    - [method with generic](#method-with-generic)

> 泛型: 提升代码复用能力，使其适用于多种数据类型

## function with generic

example: 仅仅适用于`i32`元素的容器

```rs
fn get_largest1(list: &[i32]) -> i32 {
    // i32 slice
    let mut largest = &list[0]; // largest: &i32
    for item in list {
        // item: &i32
        if item > largest {
            largest = item;
        }
    }
    *largest
}

fn get_largest2(list: &[i32]) -> i32 {
    // i32 slice
    let mut largest = list[0]; // largest: i32
    for &item in list {
        // item: i32
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn get_largest3(list: &[i32]) -> i32 {
    // i32 slice
    let mut largest = list[0]; // largest: i32
    for item in list {
        // item: &i32
        if *item > largest {
            largest = *item;
        }
    }
    largest
}
fn main() {
    let v1 = vec![11, 2, 33, 4, 5];
    let l1 = [11, 2, 3, 44, 5];
    let n1 = get_largest1(&v1);
    let n2 = get_largest2(&v1);
    let n3 = get_largest3(&v1);
    let n11 = get_largest1(&l1);
    let n22 = get_largest2(&l1);
    let n33 = get_largest3(&l1);
    println!("largest={}", n1);
    println!("largest={}", n2);
    println!("largest={}", n3);
    println!("largest={}", n11);
    println!("largest={}", n22);
    println!("largest={}", n33);
}
```

example: modify with generic

```rs
fn get_largest2<T>(list: &[T]) -> T { //这一行进行泛型改造
    // T slice
    let mut largest = list[0]; // largest: T
    for &item in list {
        // item: T
        if item > largest { // >尚未通过trait改造
            largest = item;
        }
    }
    largest
}
```

## struct with generic

```rs
struct Point<T> {
    x: T,
    y: T,
}

struct Info<T, U> {
    x: T,
    y: U,
}

fn main() {
    let s1 = Point { x: 10, y: 20 };
    let s2 = Point { x: 10.1, y: 20.2 };

    let i1 = Info { x: 10, y: 20 };
    let i2 = Info { x: 10, y: 20.1 };
    let i3 = Info { x: 10.1, y: 20.1 };
    let i4 = Info { x: 10.1, y: 20 };
}
```

### enum with generic

比如`Option<T>`, `Result<T, E>`

```rs
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### method with generic

```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn func(&self) -> &f64 {
        &self.y
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    println!("{}", p1.x()); //10
    let p2 = Point { x: 10.1, y: 20.1 };
    println!("{}", p2.func()); // 20.1
}
```

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20.2 }; // <i32, f64>
    let p2 = Point { x: "hello", y: 'c' }; // <&str, char>
    let p3 = p1.mixup(p2);
    println!("{}-{}", p3.x, p3.y); // 10-c
}
```