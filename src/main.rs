use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let _http_request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let content = fs::read_to_string("index.html").unwrap();

    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{content}", content.len()).as_bytes()).unwrap();
}
