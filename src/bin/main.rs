use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;



fn main() {
     let pool = ThreadPool::new(4);
     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("../../hello.html").unwrap();

//    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
//    stream.write(response.as_bytes()).unwrap();

    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response).unwrap();
    stream.write(contents.as_bytes()).unwrap();

    stream.flush().unwrap();
}
