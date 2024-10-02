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
    let mut reader = BufReader::new(&mut stream);

    let [method, uri] = request_info(&mut reader);
    match method.as_str() {
        "GET" => {
            let (code, file) = match uri.as_str() {
                "/" => ("200 OK", "index.html"),
                "/sleep" => {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    ("200 OK", "index.html")
                }
                _ => ("404 NOT FOUND", "404.html"),
            };


            let content = fs::read(file).unwrap();
            stream.write_all(&[
                format!("\
HTTP/1.1 {code}\r\n\
Content-Length: {}\r\n\r\n", content.len()).as_bytes(),
                &content,
            ].concat()).unwrap();
        }
        _ => return (),
    }
}

const SPACE: u8 = " ".as_bytes()[0];
/// Get the method and uri from the given HTTP request.
fn request_info(reader: &mut BufReader<&mut TcpStream>) -> [String; 2] {
    let mut info = [Vec::new(), Vec::new()].into_iter().map(|mut word| {
        reader.read_until(SPACE, &mut word).unwrap();
        word.pop();

        String::from_utf8(word).unwrap()
    });

    [info.next().unwrap(), info.next().unwrap()]
}
