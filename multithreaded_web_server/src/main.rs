use std::io::prelude::*; // get access to traits that allow reading and writing from a stream
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    // the bind() function returns Result<T, E> meaning it might fail, so we unwrap to stop the program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() { // incoming() returns an iterator of streams of type TcpStream (an open client/server connection)
        let stream = stream.unwrap(); //

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // declare a buffer on the stack to hold the data that is read in
    stream.read(&mut buffer).unwrap();

    // HTTP request format: Method URI HTTP-Version CRLF headers CRLF Body
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 Ok\r\n\r\n{}", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
