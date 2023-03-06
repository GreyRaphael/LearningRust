# Data Structure

- [Data Structure](#data-structure)
  - [struct](#struct)
    - [struct method](#struct-method)
  - [enum](#enum)

## struct

- 一旦struct的实例是`mut`，那么每一个成员都是`mut`
- struct可以作为函数返回值

```rs
struct User {
    username: String,
    age: u8,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        // 可以不按照顺序初始化
        age: 10,
        active: true,
        sign_in_count: 556,
        username: String::from("James"),
    };

    println!("{}", user1.username);
    user1.age += 20;
    println!("{}", user1.age); // 30
}

fn build_user(username:String, age:u8)->User {
    User{
        age, // 参数和struct成员字段同名，可以直接写字段名
        username,
        active:true,
        sign_in_count:668,
    }
}
```

- stuct更新语法糖: 用user1去更新user2

```rs
struct User {
    username: String,
    age: u8,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        // 可以不按照顺序初始化
        age: 10,
        active: true,
        sign_in_count: 556,
        username: String::from("James"),
    };

    let user2 = User {
        username: String::from("Tom"),
        age: 23,
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    println!("{}", user2.sign_in_count);

    let user3 = User {
        username: String::from("Jerry"),
        age: 25,
        ..user1 // 语法糖
    };
    println!("{}", user3.sign_in_count);
}
```

Tuple Struct
- 整体有一个名字，但里面的元素没有名字

```rs
fn main() {
    // Tuplt struct
    struct Color(u8, u8, u8);
    struct Point(u8, u8, u8); // Point和Color是不同类型

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

调试struct

```rs
fn main() {
    let r = Rectangle {
        width: 20,
        height: 30,
    };
    println!("{}", area(&r));

    // print Rectangle
    // println!("{}", r); // error
    println!("{:?}", r); // format 1, Rectangle { width: 20, height: 30 }
    println!("{:#?}", r); // format 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

### struct method

> 参数可以是不可变引用`&self`， 也可以是可变引用`&mut self`，也可以获取其所有权`self`,和其他参数的用法一样

```rs
fn main() {
    let r = Rectangle {
        width: 20,
        height: 30,
    };
    println!("{}", r.area());

    println!("{:#?}", r);
}

#[derive(Debug)] // 通过这一句能够实现打印struct
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用impl实现方法，方法第一个参数是self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

关联函数，类似C++里面的静态方法

```rs
fn main() {
    let r1 = Rectangle {
        width: 20,
        height: 30,
    };
    let r2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{}", r1.can_hold(&r2)); //true

    let square1 = Rectangle::square(10);
    println!("{},{}", square1.width, square1.height); // 10,10
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：通过.来访问的函数
    fn can_hold(&self, other: &Rectangle) -> bool {
        // 判断一个长方形是否能够容纳另一个长方形
        self.width > other.width && self.height > other.height
    }

    // 关联函数，通过::来使用的函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // 同一个struct的方法可以放在多个impl里面
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

## enum

```rs
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);

    // enum in struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}
```

```rs
fn main() {
    let home = NewIpAddr2::V4(127, 0, 0, 1);
    let loopbck = NewIpAddr2::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move { x: 10, y: 20 };
    let w = Message::Write(String::from("hello"));
    let red = Message::ChangeColor(255, 0, 0);
}

enum NewIpAddr1 {
    // enum里面每个成员叫做变体
    // 优点1:不需要额外的struct来存储数据
    V4(String),
    V6(String),
}

enum NewIpAddr2 {
    // 优点2:每个变体可以拥有不同的类型
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // 变体的类型可以是任意类型，也可以是struct
    Quit,
    Move { x: i32, y: i32 }, // 类型为匿名struct
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

implement method for enum

```rs
fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 10, y: 20 };
    let w = Message::Write(String::from("hello"));
    let red = Message::ChangeColor(255, 0, 0);

    red.call();
}


enum Message {
    // 变体的类型可以是任意类型，也可以是struct
    Quit,
    Move { x: i32, y: i32 }, // 类型为匿名struct
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        
    }
}
```

