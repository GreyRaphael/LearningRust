# Patterns and Matching

- [Patterns and Matching](#patterns-and-matching)
  - [`match`](#match)
  - [`if let`](#if-let)
  - [`while let`](#while-let)
  - [`for`](#for)
  - [`let`](#let)
  - [function arguments](#function-arguments)
  - [match pattern grammar](#match-pattern-grammar)
    - [specifier `_`](#specifier-_)

模式匹配类型
- 可失败的: `match`, `if let`, `while let`
- 不可失败的: `let`, `for`, 函数参数

```rs
fn main(){
    let a: Option<i32> = Some(10);
    // let Some(x)=a; // error, let只接收不可失败的，但是Some是可失败的,要用if let
    if let Some(x)=a{}
}
```

## `match`

- 需要穷尽所有可能

```rs
match VALUE {
    PATTERN1=>EXPRESSION1,
    PATTERN2=>EXPRESSION2,
    PATTERN3=>EXPRESSION3,
}
```

## `if let`

- 不会检查穷尽性

```rs
fn main(){
    let favorite_color:Option<&str> = None;
    let is_friday=false;
    let age: Result<u8,_>="23".parse();

    if let Some(c)=favorite_color{
        println!("favorite color={}", c)
    } else if is_friday{
        println!("today is friday")
    } else if let Ok(x)=age{
        if x>35{
            println!("too old")
        }else{
            println!("good labor")
        }
    } else {
        println!("use blue")
    }
}
```

## `while let`

```rs
fn main(){
    let mut stack=vec![1, 2, 3, 4, 5];
    while let Some(top)=stack.pop(){
        println!("{}", top);
    } // 5, 4, 3, 2, 1
}
```

## `for`

```rs
fn main(){
    let v=vec![1, 2, 3, 4, 5];
    for (index, value) in v.iter().enumerate(){
        println!("index={},value={}", index, value)
    }
}
```

## `let`

`let PATTERN=EXPRESSION`, let本质也是模式匹配

```rs
fn main(){
    let a=6;
    let (x, y, z)=(1, 2, 3);
}
```

## function arguments

函数参数本质也是模式匹配

```rs
fn main(){
    let point=(3, 5);
    print_coordinates(&point);
}

fn foo(x:i32){
    println!("{}", x);
}

fn print_coordinates(&(x, y) :&(i32, i32)){
    println!("coor=({},{})", x, y);
}
```

## match pattern grammar

匹配字面值

```rs
fn main(){
    let x=1;
    match x{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("anything"),
    }
}
```

匹配变量，比如下面的`Some(y)`, y是新变量和前面的y不同

```rs
fn main(){
    let x=Some(6);
    let y=10;
    match x{
        Some(50)=>println!("got 50"),
        Some(y)=>println!("got {:?}", y),
        _=>println!("default {:?}", x),
    }
    println!("x={:?}, y={:?}", x, y); // x=Some(6), y=10
}
```

多重匹配模式: `|`

```rs
fn main(){
    let x=1;
    match x{
        1|2=>println!("one or two"),
        3=>println!("three"),
        _=>println!("anything"),
    }
}
```

匹配一个范围: `..`

```rs
fn main(){
    let x=1;
    match x{
        1..=5=>println!("one,two,three,four or five"),
        6=>println!("six"),
        _=>println!("anything"),
    }

    let x='c';
    match x {
        'a'..='z'=>println!("lower case"),
        'A'..='Z'=>println!("upper case"),
        _=>println!("other symbols"),
    }
}
```

解构struct, enum, tuple, 从而引用这些类型值的部分数据

```rs
struct Point{
    x:i32,
    y:i32,
}

fn main(){
    let p1=Point{x:10, y:20};
    let Point{x: a, y:b}=p1;
    println!("a={}, b={}", a, b);

    // 简写
    let Point{x, y}=p1;
    println!("x={}, y={}", x, y);

    match p1 {
        Point{x, y:0}=>println!("on the x axis, x={}", x),
        Point{x:0, y}=>println!("on the y axis, y={}", y),
        Point{x, y}=>println!("x={}, y={}", x, y),
    }
}
```

```rs
enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    Color(u8, u8, u8),
}

fn main(){
    let msg=Message::Color(0, 1, 255);
    match msg{
        Message::Quit=>println!("quit"),
        Message::Move{x, y}=>println!("x={}, y={}", x, y),
        Message::Write(txt)=>println!("txt={}", txt),
        Message::Color(r, g, b)=>{
            println!("red={}, green={}, blue={}", r, g, b);
        }
    }
}
```

解构嵌套enum

```rs
enum ColorStyle{
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
}

enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    Color(ColorStyle),
}

fn main(){
    let msg=Message::Color(ColorStyle::RGB(0, 1, 255));
    match msg{
        Message::Color(ColorStyle::RGB(r, g, b))=>{
            println!("red={}, green={}, blue={}", r, g, b);
        },
        Message::Color(ColorStyle::HSV(h, s, v))=>{
            println!("hue={}, saturation={}, value={}", h, s, v);
        },
        _=>(),
    }
}
```

解构tuple

```rs
struct Point{
    x:i32,
    y:i32,
}

fn main(){
    let ((a, b), Point{x, y})=((10, 20), Point{x:2, y:20});
}
```

### specifier `_`

```rs
// _忽略函数参数
fn foo(_: i32, y:i32){
    println!("y={}", y);
}

fn main(){
    foo(3, 4);

    let mut v1=Some(5);
    let v2=Some(10);
    match (v1, v2){
        // _忽略值的一部分
        (Some(_), Some(_))=>println!("two value is Some"),
        _=>println!("two value eithor is Some"),
    }

    let numbers=(1, 2, 3, 4);
    match numbers {
        // _忽略值的一部分
        (first, _, third, _)=>println!("{}, {}", first, third),
        _=>(),
    }

    // _忽略未被使用的变量
    let _post=10;

    let v3=Some(1000);
    if let Some(_v)=v3{
        println!("found a value");
    }
    println!("{:?}", v3); // Some(1000)

    let v4=Some(String::from("hello"));
    if let Some(_v)=v4{
        println!("found a value");
    }
    // println!("{:?}", v4); // error, v4被_v借用了, 丧失所有权

    let v5=Some(String::from("hello"));
    if let Some(_)=v5{
        println!("found a value");
    }
    println!("{:?}", v5); // ok, v5未被_借用
}
```