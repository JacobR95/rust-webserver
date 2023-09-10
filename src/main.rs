use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    stream.write_all(format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.exec(|| {
            handle_connection(stream)
        });
    }

    println!("Shutting down.");
}
