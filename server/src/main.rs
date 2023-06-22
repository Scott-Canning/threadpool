// [server] main.rs
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // handle incoming TCP requests
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request[0]);
    println!("Request: {:#?}", fibonacci((http_request[0].parse::<u128>()).unwrap()));
}