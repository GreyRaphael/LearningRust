use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multi_thread_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // 使用线程池设置线程数量，防止DOS攻击，创建大量的线程，使得服务器崩溃

    // for stream in listener.incoming() {
    for stream in listener.incoming().take(3) {
        // take(3)只处理3个请求就关机
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down!")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request[0]);

    let (status_line, filename) = if http_request[0].starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if http_request[0].starts_with("GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
