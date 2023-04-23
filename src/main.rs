use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::time::Duration;
use hello_rust::ThreadPool;

const HTTP_SPEC: &str = "HTTP/1.1";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", request_line);

    let (status_line, filepath) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "src/hello.html")
        }
        _ => ("404 Not Found", "src/404.html"),
    };

    let content = fs::read_to_string(filepath).unwrap();
    let length = content.len();
    let response = format!("{HTTP_SPEC} {status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
