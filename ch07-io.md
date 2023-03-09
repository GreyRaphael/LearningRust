# IO

- [IO](#io)
  - [command line arguments](#command-line-arguments)
  - [refractor](#refractor)
  - [Test-Driven Development](#test-driven-development)


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

## refractor

rust对于二进制程序关注点分离的原则
> 将程序拆分成`main.rs`, `lib.rs`， 将业务逻辑放在`lib.rs`  
> 同时`lib.rs`更加方便测试

```rs
// lib.rs
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("config={:?}", config);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

```rs
// lib.rs
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("{}", contents);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // return value is tuple
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}
```

## Test-Driven Development

```rs
// man.rs
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("config={:?}", config);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

```rs
// lib.rs
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // return value is tuple
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick threes.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

