# Basic Grammar

- [Basic Grammar](#basic-grammar)
  - [Variable](#variable)
    - [Shadowing](#shadowing)
  - [DataTypes](#datatypes)
    - [integer value](#integer-value)
    - [complex](#complex)
    - [float value](#float-value)
    - [bool value](#bool-value)
    - [character value](#character-value)
    - [tuple, list, vector](#tuple-list-vector)
    - [String](#string)
    - [datatype conversion](#datatype-conversion)
  - [Statements](#statements)
    - [function, expression](#function-expression)
    - [if-else](#if-else)
    - [loop, while, for](#loop-while-for)
    - [match](#match)
    - [`if let`](#if-let)
  - [Ownership](#ownership)
    - [ownership with function](#ownership-with-function)
    - [reference](#reference)
    - [slice](#slice)
  - [error handle](#error-handle)
    - [`panic!`](#panic)
    - [`Result<T, E>`](#resultt-e)
    - [`unwrap`](#unwrap)
    - [return error](#return-error)

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

    let y:&str1 = "hello";
    let y:usize = y.len();

    let z:&str1="123456";
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

example: unsigned integer overflow

```rs
fn main() {
    let a: u8 = 255;
    // let b = a + 1; // error, will overflow
    let b = a.wrapping_add(1);
    println!("{}", b); // 0
}
```

### complex

> by 3rd lib [num](https://crates.io/crates/num)

```rs
use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
```

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
    let x = 'a';
    let y = '😂';
    let z = '我';
    println!("x={}, in memory: {} bytes", x, std::mem::size_of_val(&x)); // 4 bytes
    println!("y={}, in memory: {} bytes", y, std::mem::size_of_val(&y)); // 4 bytes
    println!("z={}, in memory: {} bytes", z, std::mem::size_of_val(&z)); // 4 bytes

    // String本质是Vec<u8>
    let mut s1 = String::new();
    s1.push(z);
    // 在stack上指针的size, 包含了3部分: addr, capacity, length, 每一个8bytes，所以24bytes
    println!("s1={}, in memory: {} bytes", s1, std::mem::size_of_val(&s1)); // 24 bytes
    // 在heap上的占用的内存大小
    println!("s1={}, in memory: {} bytes", s1, s1.bytes().len()); // 3 bytes

    for _ in 0..10{
        s1.push(z);
    }
    println!("s1={}, in memory: {} bytes", s1, std::mem::size_of_val(&s1)); // 24 bytes
    println!("s1={}, in memory: {} bytes", s1, s1.bytes().len()); // 33 bytes
    println!("s1={}, in memory: {} bytes", s1, std::mem::size_of_val(&s1[..])); // 33 bytes
    println!("s1={}, in memory: {} bytes", s1, std::mem::size_of_val(s1.as_str())); // 33 bytes
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

数组元素为未实现Copy trait的类型

```rs
fn main() {
    let a1 = [5; 3]; // [5, 5, 5]

    // let a2=[String::from("hello"); 3]; // error
    let a3 = [
        String::from("hello"),
        String::from("hello"),
        String::from("hello"),
    ];
    // 简写
    let a4: [String; 3] = core::array::from_fn(|i| String::from("hello"));

    dbg!(&a1);
    // dbg!(&a2);
    dbg!(&a3);
    dbg!(&a4);
}
```

2D Array
> 对于实现了 copy 特征的数组(例如 [i32; 10] )而言， `for n in a` 并不会把 a 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 a 。

```rs
fn main() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
```

### String

```rs
fn main() {
    // String在heap上面分配
    let mut s1 = String::from("hello");
    s1.push_str(",world");
    println!("{}", s1); // hello,world
}
```

### datatype conversion

`as`

```rs
fn main() {
   let a = 3.1 as i8;
   let b = 100_i8 as i32;
   let c = 'a' as u8; // 将字符'a'转换为整数，97

   println!("{},{},{}",a,b,c)
}
```

```rs
fn main() {
    let mut values: [i32; 5] = [1, 2, 3, 4, 5];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    print!("{:?}", values); // [1, 3, 3, 4, 5]
}
```

`try_into`

```rs
fn main() {
    let a: u8 = 10;
    let b: u16 = 100;
    // let b: u16 = 1500; // error

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
```

example: enum->number, number->enum

```rs
enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

fn main() {
    // enum to number
    let x = MyEnum::C as i32;
    // let x = 10;
    println!("x = {}", x);

    // number to enum
    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}
```

by matching

```rs
struct Foo {
    x: u32,
    y: u16,
}

#[derive(Debug)]
struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
fn main() {
    let f = Foo { x: 1, y: 2 };
    let b = reinterpret(f);
    println!("{:?}", b);
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

example: get largest

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

### match

```rs
fn main() {
    let v = Coin::Quarter;
    println!("{}", value_in_cents(v)); // 25
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // 多行代码的情况
            println!("Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

```rs
fn main() {
    let v = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(v)); // 25
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 拿到Quater里面存的值
            println!("State quarter from {:?}", state);
            25
        }
    }
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

```rs
fn main() {
    let v = 0u8;
    match v {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // 原则上u8应该256个数，需要穷举所有，现在只需要穷举1,2,3，所以用通配符_代替其他的
        _ => (),
    }
}
```

### `if let`

```rs
fn main() {
    let v = Some(0u8);

    // if let 和这个match等价，只处理一个分支的情况，放弃了穷举的可能
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    }
}
```

```rs
fn main() {
    let v = Some(0u8);

    // if let 和这个match等价，只处理两个分支的情况
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    if let Some(3) = v {
        println!("three");
    } else{
        println!("others");
    }
}
```

## Ownership

所有权规则:
1. 每个值都有一个变量，这个变量是该值的所有者
2. 每个值同时只能有一个所有者
3. 当所有者超过作用域(Scope)的时候，该值将被删除

```rs
fn main() {
    let mut s=String::from("hello");
    s.push_str(",world");
    println!("{}", s);
} // s走出作用域(Scope)，调用drop函数，清理heap上面的内存
```

```rs
fn main() {
    let x = 5;
    let y = x;
    println!("{}", x); // 5, 因为x在stack分配，没有所有权的问题，在heap上分配才有所有权的问题
    println!("{}", y); // 5
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error, 因为String在heap上分配，s1所有权转移到了s2(move), s1被Drop
    println!("{}", s2);
    
    let s3=s2.clone(); // 使用clone就会在heap上面再分配一块内存给s3，相当于是deep copy
    println!("{}", s2);
    println!("{}", s3);
}
```

概念:
- 在stack上面复制数据叫做copy
- 在heap上面复制数据叫做clone

Copy Trait: 用于能够完全存放在stack上面的数据类型
- 如果一个类型实现了Copy Trait, 那么旧变量在赋值后，仍然能够使用，比如上面的x
- 如果一个类型或者该类型的一部分实现了Drop Trait，那么该类型无法实现Copy Trait

拥有Copy Trait的类型:
- 任何简单标量的组合类型都是可以Copy的: integer, float, bool, char
- 需要分配内存或者某种资源的都不是Copy的
  1. Tuple里面所有元素都是Copy的，那么就是Copy的: (i32, f64);
  2. Tuple里面存在不是Copy的，那么就不是Copy的: (i32, String)



### ownership with function

> 将值传递给函数, Copy Trait发生复制，Drop Trait发生所有权移动

```rs
fn main() {
    let s=String::from("hello");
    take_ownership(s);
    // println!("{}", s); // error

    let x=1;
    make_copy(x);
    println!("{}", x); // x still work
}

fn take_ownership(str1:String) {
    println!("take {}", str1);
}// str1在这里被Drop

fn make_copy(num:i32) {
    println!("make copy {}", num)
}
```

> 函数返回的时候，也会发生所有权的移动

```rs
fn main() {
    let s1=give_ownership();
    println!("{}", s1);
    let s2=String::from("hello");
    println!("{}", s2); 
    let s3=take_and_giveback(s2);
    // println!("{}", s2); // s2丧失所有权
    println!("{}", s3);
}


fn give_ownership()->String {
    let str1=String::from("hello");
    str1
}


fn take_and_giveback(mut str1:String)->String {
    str1.push_str(",world");
    str1
}
```

一个变量的所有权总是遵循同样的模式
- 把一个值赋给其他变量时就会发生移动
- 当一个包含heap数据的变量离开作用域时，它的值就会被drop函数清理，除非数据所有权移动到另一个变量上

### reference

example: `mut` with ownership

```rs
fn main() {
    let x1 = 32;
    let x2 = 400;
    let x3=1000;
    
    // y可以改变自身的值,因为mut y，但是无法改变x1的值，因为不是&mut x1
    let mut y = &x1;
    println!("{}",y); // 32
    y = &x2;
    println!("{}",y); // 400

    // z无法改变自身的值，因为不是mut，但是可以改变y的值，因为是&mut y
    let z = &mut y;
    *z=&x3;
    // println!("{}", y); // cannot borrow as immutable because it is borrowed as mutable
    println!("{}", z); // 1000
}
```

使用完s1，s1的所有权发生转移

```rs
fn main() {
    let s1 = String::from("hello");
    println!("{}", s1);
    let (s2, len) = take_and_giveback(s1);

    // println!("{}, {}", s1, len); // error, s1丧失所有权
    println!("{}, {}", s2, len);
}

fn take_and_giveback(str1: String) -> (String, usize) {
    let length = str1.len();
    (str1, length) // length的类型是usize
}
```

采用Reference, 使用完s1，s1的所有权发生不发生转移
> <img src="resources/ch01-reference01.jpg" width="600">

```rs
fn main() {
    let s1 = String::from("hello");
    println!("{}", s1);
    let len = calc_length(&s1);
    println!("{}", s1); // 使用了s1的值，但是s1的所有权在函数中未发生转移

    println!("{}", len); //5
}

fn calc_length(s: &String) -> usize { 
    s.len()
}
```

```rs
// 可变引用
fn main() {
    let mut s1 = String::from("hello");
    println!("{}", s1);
    let len = calc_length(&s1);
    println!("{}", s1); // 使用了s1的值，但是s1的所有权在函数中未发生转移

    println!("{}", len); //5
}

fn calc_length(str1: &mut String) -> usize { //表示引用的内容可以被修改
    str1.len()
}
```

可变引用的限制1：在特定的作用域内，对于某一块数据，只能有一个**可变引用**
> 在编译的时候，防止数据竞争

```rs
fn main() {
    let mut s = String::from("hello");
    let s1=&mut s;
    // let s2=&mut s;// error, 只能有一个可变引用

    println!("{},{}", s1, s2);
}
```

数据竞争发生的形式:
- 两个或者多个指针同时访问同一个数据
- 至少有一个指针用于写入数据
- 没有使用任何机制来同步对数据的访问

可以通过创建新的作用域，来允许非同时的创建多个**可变引用**

```rs
fn main() {
    let mut s = String::from("hello");
    {
      let s1=&mut s;
      println!("{}", s1);
    }

    let s2=&mut s;// error, 只能有一个可变引用
    println!("{}", s1);
}
```

可变引用的限制2: 不可以同时拥有一个**可变引用**  和 一个**不可变引用**
> 多个**不可变引用**是允许的

```rs
fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // error

    println!("{},{},{}", s1, s2, s3);
}
```

```rs
// since Rust 1.31
fn main() {
   let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束
```

悬空指针(Dangling Reference): 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用
> Rust编译器能够保证引用永远都不是悬空指针：如果你引用了某些数据，编译器将保证在引用离开作用域之前，数据不会被销毁

s1离开作用域，对应的heap内存数据被销毁，但是返回了s1的引用s, s这个引用是失效的
> <img src="resources/ch01-reference01.jpg" width="600">

```rs
// 无法编译通过，
fn main() {
    let s=dangle();
}

fn dangle()->&String {
    let s1=String::from("hello");
    &s1
} // s1出了这一块就被销毁了，那么r将引用一个被销毁的数据，这就是悬空指针，rust编译器不允许
```

引用的规则
- 在任何时刻，只能满足下列条件之一
  - 一个可变引用
  - 任意数量不可变引用
- 引用必须一直有效
  - 比如上面`str1`在heap上面的数据被销毁了，但是返回了`&str1`，这个使用`&str1`不满足有效了

### slice

> 除了reference, 不持有所有权的数据类型: slice; slice本质是指向某一段数据的指针

slice用`[T]`表示
- `&[T]`, read-only reference to slice
- `&mut [T]`, mutable reference to slice
- `Box<[T]>`, heap allocated slice

```rs
fn main() {
    // slice of array, data on stck
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let slice1: &[u32] = &arr[..2];

    // slice of vector, data on heap
    let vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let slice2: &[u32] = &vec[..2];

    // equal: length and type are all the same
    assert_eq!(slice1, slice2);
    // length not the same so not equal
    assert_ne!(&arr[..], &vec[..]);
}
```

vector reference vs slice

```rs
fn main() {
    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    // type is &Vec<i32>
    let ref1 = &vec;
    // type is &[u32]
    let slice1 = vec.as_slice();
    // type is &mut [u32]
    let slice2 = vec.as_mut_slice();
    // type is &[u32]
    let slice3 = &vec[..];
}
```

slice as parameter

```rs
use core::fmt;

fn main() {
    let vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    // type is &Vec<i32>
    let ref1 = &vec;
    // type is &[u32]
    let slice1 = vec.as_slice();
    // type is &[u32]
    let slice3 = &vec[..];

    {
        // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]
        print_slice1(ref1);
        print_slice1(slice1);
    }
    {
        // &Vec<T> 支持 AsRef<[T]>
        print_slice2(ref1);
        // &[T] 支持 AsRef<[T]>
        print_slice2(slice1);
        // Vec<T> 支持 AsRef<[T]>
        print_slice2(vec);
    }
}

// parameter is &[T], slice
fn print_slice1<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice2<T: AsRef<[U]>, U: fmt::Debug>(s: T) {
    let slice: &[U] = s.as_ref();
    println!("{:?}", slice);
}
```

> `String` 是一个特殊的 `Vec`，所以在 `String` 上做切片，也是一个特殊的结构 `&str`。

```rs
use core::fmt;

fn main() {
    let s: String = "你好china".into();
    let ref1: &String = &s;
    let slice1: &str = &s[..6]; // 你好
    println!("{:?}", slice1);
    let slice2: &str = &s[..];
    let slice3: &str = s.as_str();

    {
        // &String 会被解引用成 &str
        print_slice1(ref1);
        print_slice1(slice1);
    }
    {
        // &String 支持 AsRef<str>
        print_slice2(ref1);
        // &str 支持 AsRef<str>
        print_slice2(slice1);
        // String 支持 AsRef<str>
        print_slice2(s.clone());
    }
    {
        // &String 实现了 AsRef<[u8]>, result is [228, 189, 160, 229, 165, 189, 99, 104, 105, 110, 97]
        print_slice3(ref1);
        // &str 实现了 AsRef<[u8]>, result is [228, 189, 160, 229, 165, 189]
        print_slice3(slice1);
        // String 实现了 AsRef<[u8]>, reuslt is [228, 189, 160, 229, 165, 189, 99, 104, 105, 110, 97]
        print_slice3(s.clone());
    }
}

fn print_slice1(s: &str) {
    println!("{:?}", s);
}

fn print_slice2<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice3<T: AsRef<[U]>, U: fmt::Debug>(s: T) {
    let slice: &[U] = s.as_ref();
    println!("{:?}", slice);
}
```

```rs
fn main() {
    let s = String::from("hello world");
    let s1 = &s[..5]; // hello
    let world = &s[6..]; // world
    let s3:&str = &s[0..4]; // hell
    let whole_view1 = &s[0..s.len()]; // hello world
    let whole_view2 = &s[..]; // hello world

    let literal_str="hello grey";
}
```

> <img src="resources/ch01-slice01.jpg" width="300">

上文`&str`类型就是slice, `&str`是不可变引用
> 字符串字面值(比如上文literal_str)本质就是sliec, 即`&str`，直接存在在二进制程序中；因为`&str`是不可变引用，所以字符串字面值是不可变的。

```rs
fn main() {
    let mut s1 = String::from("hello grey");
    let index = first_world_index(&s1);
    s1.clear(); // 要求s1是mut
    println!("{}", index); // s1被清空，index仍然有效，如果后文索引[index]，会出现问题, 应该保证index与s1的同步性

    let mut s2 = String::from("james moriaty");
    let first_w = first_world(&s2);
    // s2.clear(); // error, 因为前文s2已经有一个不可变引用s，而s2.clear()使用的是可变引用，所以报错
    println!("{}", first_w);
}

fn first_world_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
```

一般将`&str`作为参数类型， 这样既可以接受`&String`类型，又可以接收`&str`类型
- 如果传入的是`&String`, 会先创建一个`&str`然后进行后续处理
- 如果传入的是`&str`，直接进行处理

```rs
fn main() {
    let mut s2 = String::from("james moriaty");
    let first_w = first_world(&s2); // &String
    println!("{}", first_w);
    
    let first_w2=first_world(&s2[..]); //&str
    println!("{}", first_w2);

    let s3="tim cook";
    let first_w3=first_world(s3); // &str
    println!("{}", first_w3); // tim
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
```

## error handle

错误分类
- 可恢复的错误，采用`Result<T, E>`： 比如文件未找到
- 不可恢复的错误，采用`panic!`宏: bug, 访问越界

### `panic!`

panic宏动作
- 打印错误信息
- 展开(unwind), 清理调用栈(stack)
- 退出程序

当panic发生，可选两种方案
- 默认：程序展开调用栈(工作量大)
  - rust沿着调用栈往回走
  - 清理每个遇到的函数中的数据
- 非默认：立即终止调用栈(absort)，如果想要binary更小，在`Cargo.toml`配置
  - 不进行清理，直接停止程序
  - 内存需要OS清理

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5" 

[profile.release]
panic = "abort"
```

查看详细的报错trace: 
> Linux: `RUST_BACKTRACE=1 cargo run`  
> Windows: `$env:RUST_BACKTRACE=1 ; cargo run`

```rs
fn main() {
    panic!("crash!");
}
```

或者直接配置

```rs
use std::env;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    env::set_var("RUST_BACKTRACE", "1");
    panic!("crash!");
}
```

为了获取带调试信息的回溯,直接使用`cargo run`, 而不能使用`cargo run --release`

example: struct getter

```rs
use rand::Rng;
use std::io;

struct Guess {
    value: u32,
}

impl Guess {
    fn new(num: u32) -> Guess {
        if (num < 1) || (num > 100) {
            panic!("number range should be [1, 100]");
        }
        Guess { value: num }
    }

    // getter, 如果这个模块在外部，value就是private,所以需要getter
    fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess!");
    let secret_num = rand::thread_rng().gen_range(1..101); // [1, 101)

    loop {
        let mut num = String::new(); // mutable
        io::stdin().read_line(&mut num).expect("cannot read line!");
        let num: u32 = match num.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        let guess = Guess::new(num);
        println!("{}", guess.value);

        if guess.value() > secret_num {
            println!("Too big!");
        } else if guess.value() < secret_num {
            println!("Too small!")
        } else {
            println!("You win!");
            break;
        }
    }
}
```

example: struct getter use lib.rs

```bash
src
├── lib.rs
└── main.rs
```

```rs
// lib.rs
pub mod custom_module {
    pub struct Guess {
        value: u32, // value is private
    }

    impl Guess {
        pub fn new(num: u32) -> Guess {
            if (num < 1) || (num > 100) {
                panic!("number range should be [1, 100]");
            }
            Guess { value: num }
        }

        // getter, 如果这个模块在外部，value就是private,所以需要getter
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}
```

```rs
// main.rs
// project1是Cargo.toml package下面的name
use project1::custom_module::Guess;
use rand::Rng;
use std::io;

fn main() {
    println!("Game begins!");
    let secret_num = rand::thread_rng().gen_range(1..101); // [1, 101)

    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("cannot read line!");
        let num: u32 = match num.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        let guess = Guess::new(num); //校验工作都放在Guess内部
        println!("You num={}", guess.value());

        if guess.value() > secret_num {
            println!("Too big!");
        } else if guess.value() < secret_num {
            println!("Too small!")
        } else {
            println!("You win!");
            break;
        }
    }
}
```

### `Result<T, E>`

```rs
enum Result<T, E>{
    Ok(T),
    Err(E),
}
```

Result枚举
- T: 操作成功，Ok变体里返回的数据的类型
- E: 操作失败，Err变体里返回的错误的类型

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt"); // Result<File, Error>

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error openning file: {:?}", error);
        }
    };
}
```

```rs
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt"); // Result<File, Error>

    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error crateing file: {:?}", e),
            },
            other_err => panic!("Error openning file: {:?}", other_err),
        },
    };
}
```
 
### `unwrap`

unwrap: match表达式的一种快捷方法
- 如果Result的结果是Ok, 就返回Ok里面的值
- 如果Result的结果是Err, 就`panic!`

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap(); // 如果打开失败就panic，相当于下面的例子
}
```

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt"); // Result<File, Error>

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error openning file: {:?}", error);
        }
    };
}
```

```rs
use std::fs::File;

fn main() {
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("无法打开文件"); // 自定义信息
}
```

利用`unwrap`来简洁代码

```rs
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Error creating file: {:?}", err);
            })
        } else {
            panic!("Error openning file: {:?}", error);
        }
    });
}
```

使用`unwrap`的场景

```rs
use std::net::IpAddr;

// error
fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap(); // 绝对不犯错的时候用unwrap
    println!("{}", home.is_ipv4()); // true
}
```

### return error

```rs
use std::{
    fs::File,
    io::{self, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        // mut f是为了下面read_to_string
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(e) => Err(e),
        // Ok(x)=>Ok(s),// read_to_string的Result是usize, 重新构造了Result
        // Ok(_)=>Ok(s),//采用通配符，忽略read_to_string的返回值
        _ => Ok(s), //采用通配符，忽略read_to_string的返回值
    }
}

fn main() {
    let result = read_username_from_file().unwrap();
    println!("{:?}", result); // "grey"
}
```

通过传播错误的符号`?`来返回错误
- the `?` operator can only be used in a function that returns `Result` or `Option`
- 如果`?`前面的值是Ok, 那么就作为表达式的值，然后继续执行程序
- 如果`?`前面的值是Err, 那么Err就作为这个函数的`return`返回值
- `std::convert::From`上的`from`函数用于不同错误之间的转换。`?`的错误会隐式地被`from`函数进行转换为整个函数返回的Err类型

```rs
use std::{
    fs::File,
    io::{self, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_username_from_file().unwrap();
    println!("{:?}", result); // "grey"
}
```

```rs
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式调用，简洁代码
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

- `?`只能用于返回值是`Result`类型的函数，用在默认main函数里面不行，因为main函数默认返回类型是`()`

```rs
use std::{error::Error, fs::File};

// // error
// fn main() {
//     let f = File::open("hello.txt")?;
// }

// Box<dyn Error>>是任何可能的错误类型
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```