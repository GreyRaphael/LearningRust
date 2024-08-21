# Rust use library

- [Rust use library](#rust-use-library)
  - [Rust and C++ Interoperability](#rust-and-c-interoperability)
    - [Rust Use C++ Shared Library](#rust-use-c-shared-library)
    - [Rust Use C++ Static Library](#rust-use-c-static-library)
    - [Rust Build \& Use C++ Static Library](#rust-build--use-c-static-library)
    - [C++ Use Rust Shared Library](#c-use-rust-shared-library)
    - [C++ Use Rust Static Library](#c-use-rust-static-library)
  - [tokio async](#tokio-async)

## Rust and C++ Interoperability

### Rust Use C++ Shared Library

step1: genereate a c++ shared library

```bash
mylibs
  ├── CMakeLists.txt
  └── mylib.cpp
```

```cmake
# mylibs/CMakeLists.txt
cmake_minimum_required(VERSION 3.24.0)
project(mylib VERSION 0.1.0 LANGUAGES C CXX)

add_library(mylib SHARED mylib.cpp)
```

```cpp
// mylibs/mylib.cpp
extern "C" {
int add_numbers(int x, int y);
}

int add_numbers(int x, int y) {
    return x + y;
}
```

step2: generate a rust project
1. `cargo new proj1`
1. `mkdir libs`, move `mylibs/build/libmylib.so` to `proj1/libs/`
1. create a file `proj1/build.rs`
1. create a file `proj1/src/c_wrapper.rs`

```bash
proj1
  ├── build.rs
  ├── Cargo.toml
  ├── libs
  │   └── libmylib.so
  └── src
      ├── c_wrapper.rs
      └── main.rs
```

```toml
# proj1/Cargo.toml
[package]
name = "proj1"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2.149"
```

```rust
// proj1/src/c_wrapper.rs
#[link(name = "mylib", kind = "dylib")]
extern "C" {
    fn add_numbers(x: i32, y: i32) -> i32;
}

pub fn add_numbers_rust(x: i32, y: i32) {
    let result = unsafe { add_numbers(x, y) };
    println!("x={}, y={}, result={}", x, y, result);
}
```

```rust
// proj1/src/main.rs
mod c_wrapper;

fn main() {
    c_wrapper::add_numbers_rust(30, 20);
}
```

```rust
// proj1/build.rs
use std::fs;

fn main() {
    // use libs for link
    println!(
        "cargo:rustc-link-search=native={}/libs",
        std::env::current_dir().unwrap().display()
    );
    // set binary rpath
    println!("cargo:rustc-link-arg-bins=-Wl,-rpath,./");

    // mv files to target
    fs::copy("libs/libmylib.so", "target/debug/libmylib.so").expect("Failed to copy files");
}
```

### Rust Use C++ Static Library

step1: genereate a c++ static library

```bash
mylibs
  ├── CMakeLists.txt
  └── mylib.cpp
```

```cmake
# mylibs/CMakeLists.txt
cmake_minimum_required(VERSION 3.24.0)
project(mylib VERSION 0.1.0 LANGUAGES C CXX)

add_library(mylib mylib.cpp)
```

```cpp
// mylibs/mylib.cpp, not changed
extern "C" {
int add_numbers(int x, int y);
}

int add_numbers(int x, int y) {
    return x + y;
}
```

step2: generate a rust project
1. `cargo new proj1`
1. `mkdir libs`, move `mylibs/build/libmylib.a` to `proj1/libs/`
1. create a file `proj1/build.rs`
1. create a file `proj1/src/c_wrapper.rs`

```bash
proj1
  ├── build.rs
  ├── Cargo.toml
  ├── libs
  │   └── libmylib.a
  └── src
      ├── c_wrapper.rs
      └── main.rs
```

```rust
// proj1/build.rs
fn main() {
    // use libs for link
    println!(
        "cargo:rustc-link-search=native={}/libs",
        std::env::current_dir().unwrap().display()
    );
}
```

```rust
// proj1/src/c_wrapper.rs
#[link(name = "mylib", kind = "static")]
extern "C" {
    fn add_numbers(x: i32, y: i32) -> i32;
}

pub fn add_numbers_rust(x: i32, y: i32) {
    let result = unsafe { add_numbers(x, y) };
    println!("x={}, y={}, result={}", x, y, result);
}
```

```rust
// proj1/src/main.rs, not changed
mod c_wrapper;

fn main() {
    c_wrapper::add_numbers_rust(30, 20);
}

```

```toml
# proj1/Cargo.toml, not changed
[package]
name = "proj1"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2.149"
```

### Rust Build & Use C++ Static Library

Steps
1. `cargo new proj1`
2. create a `mylib.cpp` in `proj1/src-cpp/`
3. create a `build.rs` in `proj1/`
4. create a `c_wrapper.rs` in `proj1/src/`

```bash
proj1
  ├── build.rs
  ├── Cargo.toml
  ├── src
  │   ├── c_wrapper.rs
  │   └── main.rs
  └── src-cpp
      └── mylib.cpp
```

```toml
# proj1/Cargo.toml
[package]
name = "proj1"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2.149"

[build-dependencies]
cc = "1.0"
```

```rust
// proj1/build.rs
use cc;

fn main() {
    cc::Build::new().file("src-cpp/mylib.cpp").compile("mylib");
}
```

```rust
// proj1/src/c_wrapper.rs, can only be static
#[link(name = "mylib")]
extern "C" {
    fn add_numbers(x: i32, y: i32) -> i32;
}

pub fn add_numbers_rust(x: i32, y: i32) {
    let result = unsafe { add_numbers(x, y) };
    println!("x={}, y={}, result={}", x, y, result);
}
```

```rust
// proj1/src/main.rs, not changed
mod c_wrapper;

fn main() {
    c_wrapper::add_numbers_rust(30, 20);
}
```

```cpp
// src-cpp/mylib.cpp, hot changed
extern "C" {
int add_numbers(int x, int y);
}

int add_numbers(int x, int y) {
    return x + y;
}
```

### C++ Use Rust Shared Library

> Tutorial: [A little Rust with your C](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html)

step1: rust buld a shared library
1. `cargo new rust-lib --lib`
2. write rust functions

```bash
rust-lib
  ├── Cargo.toml
  └── src
      └── lib.rs
```

```toml
# rust-lib/Cargo.toml
[package]
name = "rust-lib"
version = "0.1.0"
edition = "2021"

[lib]
name = "myadd"
crate-type = ["cdylib"] # Creates dynamic lib
```

```rust
// rust-lib/src/lib.rs
#[no_mangle]
pub extern "C" fn rust_function(x: i32, y: i32) -> i32 {
    println!("Hello from Rust, result = {}", x + y);
    x + y
}
```

step2: cpp use rust shared library
1. create a cmake project
2. write header file for rust shared library. There is a tool to automate this process, called [cbindgen](https://github.com/mozilla/cbindgen) which analyses your Rust code and then generates headers for your C and C++ projects from it, `cbindgen --config cbindgen.toml --crate rust-libs --output myadd.h`, where `cbindgen.toml` is empty file
3. config `CMakeLists.txt`
4. invoke rust functions

```bash
proj2
  ├── CMakeLists.txt
  ├── main.cpp
  └── rust-lib
      ├── libmyadd.so
      └── myadd.h
```

```h
// proj2/rust-lib/myadd.h
extern "C" {

int rust_function(int x, int y);

}
```

```cmake
# proj2/CMakeLists.txt
cmake_minimum_required(VERSION 3.24.0)
project(proj2 VERSION 0.1.0 LANGUAGES C CXX)

add_executable(proj2 main.cpp)

target_include_directories(proj2 PRIVATE ${CMAKE_SOURCE_DIR}/rust-lib)
target_link_libraries(proj2 PRIVATE ${CMAKE_SOURCE_DIR}/rust-lib/libmyadd.so)
```

```cpp
// proj2/main.cpp
#include <iostream>

#include "myadd.h"

int main(int, char**) {
    auto result = rust_function(100, 200);
    std::cout << result << '\n';
}
```

### C++ Use Rust Static Library

step1: rust buld a static library
1. `cargo new rust-lib --lib`
2. write rust functions

```bash
rust-lib
  ├── Cargo.toml
  └── src
      └── lib.rs
```

```toml
# rust-lib/Cargo.toml
[package]
name = "rust-libs"
version = "0.1.0"
edition = "2021"

[lib]
name = "myadd"
# crate-type = ["cdylib"] # Creates dynamic lib
crate-type = ["staticlib"] # Creates static lib
```

```rust
// rust-lib/src/lib.rs, not changed
#[no_mangle]
pub extern "C" fn rust_function(x: i32, y: i32) -> i32 {
    println!("Hello from Rust, result = {}", x + y);
    x + y
}
```

step2: cpp use rust shared library
1. create a cmake project
2. write header file for rust static library
3. config `CMakeLists.txt`
4. invoke rust functions

```bash
proj2
  ├── CMakeLists.txt
  ├── main.cpp
  └── rust-lib
      ├── libmyadd.so
      └── myadd.h
```

```h
// proj2/rust-lib/myadd.h, unchanged
extern "C" {

int rust_function(int x, int y);

}
```

```cmake
# proj2/CMakeLists.txt
cmake_minimum_required(VERSION 3.24.0)
project(proj2 VERSION 0.1.0 LANGUAGES C CXX)

add_executable(proj2 main.cpp)

target_include_directories(proj2 PRIVATE ${CMAKE_SOURCE_DIR}/rust-lib)
target_link_libraries(proj2 PRIVATE ${CMAKE_SOURCE_DIR}/rust-lib/libmyadd.a)
```

```cpp
// proj2/main.cpp, unchanged
#include <iostream>

#include "myadd.h"

int main(int, char**) {
    auto result = rust_function(100, 200);
    std::cout << result << '\n';
}
```

## tokio async

simple example
- `cargo new proj1`, then `cd proj1`
- `cargo add tokio -F macros -F rt -F rt-multi-thread`, then edit `src/main.rs`

```rs
// main.rs
use std::{thread::sleep, time::Duration};

// #[tokio::main] // default multi-thread
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main(flavor = "current_thread")] // single thread
async fn main() {
    println!("begin tasks");
    let value = 100;

    let job1 = tokio::spawn(async move {
        let ret = task1(value).await;
        println!("job1 get {}", ret);
    });
    let job2 = tokio::spawn(async move {
        let ret = task2(value).await;
        println!("job2 get {}", ret);
    });

    let _ = tokio::join!(job1, job2);
}

async fn task1(x: i32) -> i32 {
    sleep(Duration::new(2, 0));
    println!("task 1 costs 2s");
    111111 + x
}

async fn task2(x: i32) -> i32 {
    sleep(Duration::new(4, 0));
    println!("task 2 costs 4s");
    2222 + x
}
```