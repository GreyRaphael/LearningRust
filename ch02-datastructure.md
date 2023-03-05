# Data Structure

- [Data Structure](#data-structure)
  - [struct](#struct)

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