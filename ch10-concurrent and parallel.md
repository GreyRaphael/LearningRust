# Rust Concurrency and Parallel

- [Rust Concurrency and Parallel](#rust-concurrency-and-parallel)
  - [Thread](#thread)

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

