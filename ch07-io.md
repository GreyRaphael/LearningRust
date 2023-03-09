# IO

- [IO](#io)

## command line arguments

```rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("target={}, fileame={}", query, filename);

    let contents = fs::read_to_string(filename).expect("cannot read file");

    println!("{}", contents);
}
```

