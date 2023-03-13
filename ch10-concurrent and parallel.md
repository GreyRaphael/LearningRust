# Rust Concurrency and Parallel

- [Rust Concurrency and Parallel](#rust-concurrency-and-parallel)
  - [Thread](#thread)
  - [Message Passing](#message-passing)

## Thread

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let j_handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi numer {} in spawned thread", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    j_handler.join().unwrap();
}
```

```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi numer {} in spawned thread", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..6 {
        println!("hi number {} in mainthread", i);
        thread::sleep(Duration::from_millis(1000));
    }
}
```

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    // move强制获取v所有权
    let handle = thread::spawn(move || {
        println!("here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

## Message Passing

消息传递: 一种很流行且能保证安全并发的技术
- 线程通过彼此发送消息来进行通信
- 通过Channel来实现，标准库提供

Channel: 包含发送端和接收端
- 如果发送端、接收端任意一端被丢弃，那么Channel就关闭

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv会阻塞当前线程，知道Channel中有数据传过来
    let received = rx.recv().unwrap();
    println!("mainthread get: {}", received);
}
```