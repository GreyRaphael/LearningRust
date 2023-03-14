# Patterns and Matching

- [Patterns and Matching](#patterns-and-matching)
  - [`match`](#match)
  - [`if let`](#if-let)
  - [`while let`](#while-let)
  - [`for`](#for)
  - [`let`](#let)
  - [function arguments](#function-arguments)

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

