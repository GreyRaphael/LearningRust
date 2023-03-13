# Rust Concurrency and Parallel

- [Rust Concurrency and Parallel](#rust-concurrency-and-parallel)
  - [Thread](#thread)
  - [Message Passing](#message-passing)
  - [Shared-State Concurrency](#shared-state-concurrency)
  - [`Send`, `Sync` trait](#send-sync-trait)

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
use std::sync::mpsc; // mpsc: multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 需要使用move取得tx所有权
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv会阻塞当前线程，知道Channel中有数据传过来
    let received = rx.recv().unwrap(); // recv() return is Result<T, E>
    println!("mainthread get: {}", received);
}
```

`try_recv`, 立即返回`Result<T,E>`
- 如果有数据到达，返回`Ok`
- 否则返回`Err`
- 通常使用循环来检查`try_recv`

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Trump"),
            String::from("Biden"),
            String::from("Grey"),
            String::from("Murcy"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("mainthread get: {}", received);
    }
}
```

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("threa1: Trump"),
            String::from("threa1: Biden"),
            String::from("threa1: Grey"),
            String::from("threa1: Murcy"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("thread2: Trump"),
            String::from("thread2: Biden"),
            String::from("thread2: Grey"),
            String::from("thread2: Murcy"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("mainthread get: {}", received);
    }
}
```

## Shared-State Concurrency

通过共享内存来并发
- channel类似**单所有权**，一旦所有权转移至Channel，就无法使用原变量
- 共享状态并发类似**多所有权**，多个线程可以同时访问同一块内存

Mutex: Mutual exclusion
> 同一时刻，只允许一个线程来访问数据
- 使用数据前，必须尝试获取mutex
- 使用完mutex, 必须对数据解锁，以便其它线程可以获取锁
- Mutex::new()创建`Mutex<T>`，本质是智能指针
- `.lock()`函数返回`MutexGuard`,实现了`Dref`, `Drop`

```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard发生drop
    println!("m={:?}", m); // m=Mutex { data: 6, poisoned: false, .. }
}
```

多重所有权+多线程
> `Rc<T>` + `Thread`: 仅仅使用单线程  
> `Arc<T>` + `Thread`: Atomic

```rs
use std::{sync::Mutex, thread, vec, task::Context};

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        // error, 10个线程，第一个线程获取了counter的所有权，后面的没有获得到，所有报错
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Result={}", counter.lock().unwrap())
}
```

```rs
use std::{rc::Rc, sync::Mutex, thread, vec};

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // error, Rc仅仅适用于单线程
        let c = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Result={}", counter.lock().unwrap())
}
```

true example:

```rs
use std::{
    sync::{Arc, Mutex},
    thread, vec,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Result={}", counter.lock().unwrap())
}
```

`Mutex<T>`提供了内部可变性，和`Cell`家族一样
- 使用`RefCell<T>`改变`Rc<T>`里面的内容，有循环引用风险
- 使用`Mutex<T>`改变`Arc<T>`里面的内容，有死锁风险，

## `Send`, `Sync` trait

Rust语言本身涉及并发性较少，其并发特性来自标准库

实现`Send trait`的类型可以在线程间转移所有权，Rust几乎所有类型都实现了`Send`
> 但是`Rc<T>`没有实现`Send`，只用于单线程场景

实现`Sync trait`的类型可以安全地被多个线程引用，Rust基础类型都实现了`Send`
> 如果`T`是`Sync`，那么`&T`就是`Send`  
> 但是`Rc<T>`, `RefCell<T>`, `Cell<T>`没有实现`Sync`，只用于单线程场景  
> `Mutex<T>`实现`Sync`，可用于多线程场景  