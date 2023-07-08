use std::{
    fs,
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};

use basic_server_rust::ThreadPool;

use std::net::{TcpListener, TcpStream};

fn main() {
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        pool.execute(|| handler_func(stream));
    }
}

fn handler_func(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let get = "GET / HTTP/1.1";
    let (status_line, file_name) = if http_request[0].starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if (http_request[0].starts_with("GET /sleep HTTP/1.1")) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 BAD REQUEST", "404.html")
    };

    //let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Lenght:{length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
