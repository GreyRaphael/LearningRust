# Data Structure

- [Data Structure](#data-structure)
  - [struct](#struct)
    - [struct method](#struct-method)
  - [enum](#enum)
    - [`Option<T>`](#optiont)
  - [`Vec<T>`](#vect)
    - [vector store different type of data](#vector-store-different-type-of-data)
  - [String](#string)
    - [string operation](#string-operation)
    - [string escape](#string-escape)
    - [string visit element](#string-visit-element)
  - [`HashMap<K, V>`](#hashmapk-v)

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

struct with lifetime for `&str`

```rs
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
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

### `Option<T>`

> 表示某个值可能存在或者不存在的情况

```rs
// 因为是prelude, 所以 Option<T>, Some, None可以直接使用
enum Option<T> {
    Some(T),
    None,
}
```

```rs
fn main() {
    let v1 = Some(5);
    let v2 = Some("hello");
    let v3: Option<i32> = None; // 需要显示指定None类型
}
```

> `Option<T>`和`T`是不同类型，不能把`Option<T>`直接当作是`T`

```rs
fn main() {
    let v1 = Some(5);
    let v2: i32 = 8;
    // let sum = v1 + v2; // error
    let sum = v1.unwrap() + v2;
    println!("{}", sum);
}
```

`Option<T>` with `match`

```rs
fn main() {
    let v1 = Some(65);
    let v2 = plus_one(v1);

    println!("{}", v2.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 匹配必须穷举所有可能， 少写了None会报错
    match x {
        None => None, 
        Some(i) => Some(i + 1),
    }
}
```

## `Vec<T>`

`Vec<T>`
- 元素类型相同
- 内存中连续存放，在heap上
- 出了作用域，Vector被删除，里面的元素也被删除

```rs
fn main() {
    let mut v1 = Vec::new(); // 通过下面的push才推断出Vec<i32>
    v1.push(1);
    let mut v2: Vec<u8> = Vec::new();
    // v2.push(256); // error

    // recommended, by macros
    let v3 = vec!['a', 'b', 'c', 'd'];

    // visit method1
    let idx = 2;
    // let idx = 100;
    let third = &v3[idx];
    println!("{}", third);
    // visist method2, 超出索引引用会panic, 使用get不会panic
    match v3.get(idx) {
        Some(value) => println!("{}", value),
        None => println!("no 3rd num"),
    }
}
```

因为vector元素在内存连续分布的，如果添加一个新元素，可能原来连续的区段不够，需要释放并重新分配内存；下面的`first`的不可变引用就会失效，导致问题

```rs
fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let first = &v1[2]; // immutable ref
    v1.push(5); // error,因为已经是immutable引用，所以不能使用mutable ref, 而push调用的是mutable ref
    println!("{}", first);
}
```

### vector store different type of data

> Vector + Enum

```rs
#[derive(Debug)]
enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SheetCell::Int(3),
        SheetCell::Text(String::from("blue")),
        SheetCell::Float(12.3),
    ];

    for cell in &row {
        println!("{:?}", cell)
    }
}
```

## String

- 字符串数据结构复杂, 基于Byet的集合
- 字符串采用utf8编码

Rust**核心语言**层面，只有一种字符串类型, 即字符串切片: `&str`
字符串切片(`&str`): 对存储于其他地方的utf8编码的字符串的引用
- 字符串字面值，存储在二进制文件中，也是字符串切片

`String`类型来自**标准库**，不是核心语言
- 可增长、可修改、可拥有
- utf8编码

### string operation

```rs
fn main() {
    let s1 = String::from("I like c++. Learning c++ is my favorite!");
    let new_s1 = s1.replace("c++", "rust");
    dbg!(new_s1);
    let new_s2 = s1.replacen("c++", "rust", 1);
    dbg!(new_s2);

    let mut s2 = String::from("I like c++. Learning c++ is my favorite!");
    let new_s3 = s2.replace_range(7..8, "R");
    dbg!(new_s3); // ()
    dbg!(s2); // s2 = "I like R++. Learning c++ is my favorite!"

    let mut s3 = String::from("hello,你好");
    let c1 = s3.pop();
    let c2 = s3.pop();
    dbg!(c1); // 好
    dbg!(c2); // 你
    dbg!(s3); // hello,

    let mut s4 = String::from("你好,china");
    dbg!(std::mem::size_of_val(&s4)); // 24
    dbg!(std::mem::size_of_val(&s4[..])); // 12
    dbg!(std::mem::size_of_val(s4.as_str())); // 12
    s4.remove(0);
    dbg!(&s4); // 好,china

    s4.truncate(4);
    dbg!(&s4); // 好,

    s4.clear();
    dbg!(s4); // ""
}
```

```rs
fn main() {
    // method1
    let mut s1 = String::new();
    s1.push('a');
    s1.push('b');
    s1.push_str("你好");
    println!("{}", s1);

    // method2
    let mut s2 = "hello".to_string();
    s2.push_str("你号");
    println!("{}", s2);

    // method3
    let mut s3 = String::from("你好");
    s3.push_str("moris");
    println!("{}", s3);

    // concate string
    let s4=String::from("hello ");
    let s5=String::from("james");
    let s6=s4+&s5;
    // println!("{}", s4); 
    // s4丧失所有权, 因为调用了 fn add(self, s:&str)->String {...}
    // s4的所有权进入函数add里面，所以失效; s5是不可变引用，所以没有失效
    println!("{}", s5);
    println!("{}", s6);
}
```

```rs
fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("{}", result); // hello rust!!!!
}
```

```rs
// &s5的类型是&String, 但是被强制解引用转换成slice &str
fn add(self, s:&str)->String {

}
```

- `format!` 拼接多个字符串

```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("你好");

    // let s4 = s1 + "-" + &s2 + "-" + &s3; // s1丧失所有权
    // println!("{}", s4);

    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s5);

    // s1, s2, s3仍具有所有权
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
```

### string escape

```rs
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // What are you doing? (\x3F means ?) I'm writing Rust!

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );
    // Unicode character ℝ (U+211D) is called "DOUBLE-STRUCK CAPITAL R"

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

```rs
fn main() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

### string visit element

rust字符串不支持索引语法访问: `str1[0]`不允许
> `String`本质是对`Vec<u8>`的包装

```rs
fn main() {
    let s1 = String::from("hello, 你好");
    // let c1 = s1[0]; //error, not support Index<{integer}>
    // let c2 = &s1[0]; //error, not support Index<{integer}>
}
```

字节(Bytes)、标量值(Scalar Values)、字形簇(Grapheme Clusters)
- 其中字节就是u8, 标量值表面单字占用的字节，字形簇更加接近`字母`的概念(一般第三方提供)

```rs
fn main() {
    let s1 = String::from("hi你好");
    // "你好"unicode标量值用3个字节表示(228, 189, 160)，中文一般是2或者3
    println!("{}", s1.len()); // 8

    for b in s1.bytes() { // 获取字节
        print!("{}, ", b);
    } // 104, 105, 228, 189, 160, 229, 165, 189,

    println!(); // 换行
    for c in s1.chars() { // 获取标量值
        print!("{}, ", c);
    } // h, i, 你, 好,
}
```

获取String切片slice
- 切割必须沿着字符的边界进行切割，否则引起panic

```rs
fn main() {
    let s1 = String::from("hi你好");
    // let s2=&s1[..4];
    // println!("{}", s2); // error, 切割错误, byte index 4 is not a char boundary
    let s3 = &s1[..5];
    println!("{}", s3); // hi你
}
```

## `HashMap<K, V>`

HashMap
- HashMap不在prelude中，需要`use std::collections::HashMap;`
- 数据存储在heap上
- 内部元素同构：所有的key同一种类型，所有的value同一种类型

```rs
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("grey"), 89);
}
```

```rs
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {
    let s = Vec::from([1, 2, 3]);
    println!("{:?}", s);

    let s = BTreeSet::from([1, 2, 3]);
    println!("{:?}", s);

    let s = HashSet::from([1, 2, 3]);
    println!("{:?}", s);

    let s = BTreeMap::from([(1, 2), (3, 4)]);
    println!("{:?}", s);

    let s = HashMap::from([(String::from("grey"), 2), (String::from("James"), 4)]);
    println!("{:?}", s);
}
```

zip to crate HashMap

```rs
use std::collections::HashMap;

fn main() {
    let names = vec![
        String::from("grey"),
        String::from("tom"),
        String::from("moris"),
    ];
    let scores = vec![10, 20, 30, 40];
    let dict: HashMap<_, _> = names.iter().zip(scores.iter()).collect(); // 必须指明类型，因为collect可以返回多种类型, _ 用于自动推断

    println!("{:?}", dict); // {"moris": 30, "tom": 20, "grey": 10}
}
```

```rs
use std::collections::HashMap;

fn main() {
    let name1 = String::from("grey");
    let name2 = String::from("tom");

    let mut map1 = HashMap::new();
    map1.insert(name1, 100);
    map1.insert(name2, 200);
    println!("{:?}", map1);
    // println!("{}, {}", name1, name2); // insert之后，name1, name2失去所有权

    let name3 = String::from("grey");
    let name4 = String::from("tom");
    let mut map2 = HashMap::new();
    map2.insert(&name3, 100);
    map2.insert(&name4, 200);
    println!("{:?}", map2);
    println!("{}, {}", name3, name4); // 使用引用，insert之后，name3, name4仍具有所有权
}
```

visist hashmap

```rs
use std::collections::HashMap;

fn main() {
    let map1 = HashMap::from([(String::from("grey"), 2), (String::from("James"), 4)]);

    match map1.get("grey") {
        Some(s) => println!("score={}", s),
        None => println!("not exist!"),
    }

    for (k, v) in &map1 {
        println!("{}:{}", k, v);
    }
}
```

更新HashMap
- 每个k只对应一个v
  - k已经存在
    - 替换现有v
    - 保留现有v, 忽略新v
    - 合并现有的v和新v
  - k不存在
    - 添加新v

```rs
use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::from([(String::from("grey"), 2), (String::from("James"), 4)]);
    // upate directly
    map1.insert(String::from("grey"), 1000);
    println!("{:?}", map1); // {"James": 4, "grey": 1000}

    // if not exist, add; else, update
    map1.entry(String::from("Jerry")).or_insert(200);
    map1.entry(String::from("James")).or_insert(100);
    println!("{:?}", map1); //{"Jerry": 200, "grey": 1000, "James": 4}
}
```

>`or_insert()`返回key-value中value的可变引用

```rs
use std::collections::HashMap;

fn main() {
    let txt = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in txt.split_whitespace() {
        let value = map.entry(word).or_insert(0); // or_insert返回可变引用 &mut i32
        *value += 1;
    }

    println!("{:?}", map); // {"wonderful": 1, "world": 2, "hello": 1}
}
```