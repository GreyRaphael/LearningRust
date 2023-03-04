# Basic Grammar

- [Basic Grammar](#basic-grammar)
  - [Variable](#variable)
    - [Shadowing](#shadowing)
  - [DataTypes](#datatypes)
    - [integer value](#integer-value)
    - [float value](#float-value)
    - [bool value](#bool-value)
    - [character value](#character-value)
    - [tuple, list, vector](#tuple-list-vector)
  - [Statements](#statements)
    - [function, expression](#function-expression)
    - [if-else](#if-else)
    - [loop, while, for](#loop-while-for)
  - [Ownership](#ownership)

## Variable

变量
- 声明变量使用`let`
- 变量默认为immutable，使用`mut`关键字之后才能改变


常量: `const MAX_POINTS:u32=1000;`
- 常量不可以使用`mut`
- 常量需要`const`,类型必须指定
- 常量能够在任何作用域内声明，包括全局作用域
- 常量只能绑定到常量表达式，无法绑定到函数调用结果，或者运行时才计算的值
- 程序运行期间，常量在其声明的作用域内一直有效


```rs
const MAX_PONTS: u32 = 100_000; // is 100000, 为了增加可读性

fn main() {
    const MIN_PONTS: u32 = 100;
    let x = 5;
    // x=6;// error
    println!("x={}", x);
    let mut y = 10;
    y = 100;
    println!("y={}", y);
    println!("MAX_PONTS value={}", MAX_PONTS);
    print!("MIN_PONTS value={}", MIN_PONTS);
}
```

### Shadowing

可以使用相同名字声明新的变量，新的变量就会shadow(隐藏)之前声明的变量
- 使用`let`声明的同名新变量，也是immutable
- 使用`let`声明的同名新变量，可以是不同类型

```rs
fn main() {
    let x = 5;
    // x = x + 1; // error, x is immutable
    let x = x + 1; // shaowing

    let y = "hello";
    let y = y.len();

    let z="123456";
    let z:i32=z.parse().unwrap();
    
    println!("x={}", x); // 6
    println!("y={}", y); // 5
    println!("z={}", z); // 123456
}
```

## DataTypes

数据类型
- 标量类型: 代表一个单个的值
- 复合类型

```rs
fn main() {
    let x:i32 = 5;
    // x = x + 1; // error, x is immutable
    let x:i32 = x + 11; // shaowing

    let y:&str = "hello";
    let y:usize = y.len();

    let z:&str="123456";
    let z:i32=z.parse().unwrap();
    
    println!("x={}", x); // 6
    println!("y={}", y); // 5
    println!("z={}", z); // 123456
}
```


### integer value

- signed: from $-(2^n-1)$, to $2^{n-1}-1$
- unsigned: from 0 to $2^n-1$
- arch由计算机架构决定，如果是64位计算机，isize就是i64

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

除了`Byte`都可以使用类型后缀`98_222u32`, `98_222i32`
> 即便是在64位操作系统，i32的速度也很快

| Number Literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   | b'A'        |

### float value

- f32
- f64, 默认情况，现代计算机上f64和f32速度差不多，精度反而更高

### bool value

占用一个Byte
- true
- false

### character value

占用4个字节，unicode标量值

范围: `U+0000 ~ U+D7FF`, `U+E000 ~ U+10FFFF`

```rs
fn main() {
    let x='a';
    let y='😂';
    println!("x={}", x);
    println!("y={}", y);
}
```

### tuple, list, vector

```rs
fn main() {
    // Tuple: 元素类型可以不同
    let tup:(i32, f64,u8)=(100, 1.2, 2);
    let t=(1000, 1.3, 5);
    let (x, y, z)=t;
    println!("{},{},{}", tup.0, tup.1, tup.2);
    println!("{},{},{}", x, y, z);

    // Array, 元素类型相同，长度不许改变，数据存放在stack上
    let l=[1, 2, 3, 4, 5]; // immutable
    // 显式说是类型是f64,共3个元素
    let mut lx:[f64; 3]=[1.1, 2.2, 3.3]; // mutable
    lx[2]=20.3;
    // 重复多少次元素
    let ly=[2;3]; // [2, 2, 2]
    println!("{}", l[2]);
    println!("{}", lx[2]); // 20.3

    // Vector, 元素类型相同，长度可以改变
    let mut v : Vec<i32> = Vec::new();
    v.push(11);
    v.push(22);
    v.push(33);
    for i in v 
    { 
        print!("{} ",i); 
    } 

    let v2=vec![111, 222, 333]; // create by macro
    for i in v2
    {
        println!("{}", i)
    }
}
```

## Statements

### function, expression

```rs
fn main() {
    func1();
    func2(10);
    // 函数不需要定义在main之前
    // rust不支持函数默认参数
    println!("{}", func3()); // 10000
    println!("{}", func4(10)); // 15
    println!("{}", func5(1)); // 51
}

fn func1(){
    println!("func1");
}

fn func2(x:i32){
    println!("func2, x={}", x);
}

fn func3() -> i32{
    10000
}

fn func4(x:i32) -> i32{
    x+5 // 加了分号就是statemetn， 类型是Tuple，与返回值类型不匹配，报错
}

fn func5(x:i32) -> i32{
    return x+50;
}
```

```rs
fn main() {
    let y={ // 花括号说明是表达式
        let x=1;
        // x+3; // 加上末尾分号就是语句statement,语句的返回值是空的tuple
        x+3 // 不加末尾分号就是expression
    };
    // let z={let x=6}; // error, 认为是statement
    println!("{}", y); // 4
}
```

### if-else

```rs
fn main() {
    let x = 6;
    if x % 3 == 0 {
        print!("arm 1");
    } else if x % 3 == 1 {
        print!("arm 2");
    } else {
        print!("arm 3");
    }

    let num = if x % 2 == 0 { 100 } else { 200 }; 
    // let num = if x % 2 == 0 { 100 } else { "hello" }; // error, 两个分支必须是同种类型
    println!("num={}", num); // 100
}
```

### loop, while, for

```rs
fn main() {
    let mut counter=0;

    // 通过break跳出
    loop {
        counter+=1;
        println!("counter={}", counter);
        if counter==5{
            break;
        }
    }

    let mut x=0;
    while x<5 {
        x+=1;
        println!("x={}", x);
    }

    // 1..6表示range
    for i in 1..6 {
        println!("i={}", i);
    }

    // 逆序
    for i in (1..6).rev() {
        println!("i={}", i);
    }
}
```


```rs
fn main() {
    let mut counter = 0;

    // loop作为表达式
    let result = loop {
        counter += 1;
        if counter == 5 {
            break 2 * counter;
        }
    };

    println!("counter={}", counter); // 5
    println!("result={}", result); // 10
}
```

```rs
fn main() {
    let l = [1, 2, 3, 4, 5];
    let t=(10, 1.2, 'h', 1);
    for i in l.iter() {
        println!("value={}", i);
    }
    // 可以遍历Array, 无法遍历Tuple
}
```

## Ownership

所有权规则:
1. 每个值都有一个变量，这个变量是该值的所有者
2. 每个值同时只能有一个所有者
3. 当所有者超过作用域(Scope)的时候，该值将被删除

