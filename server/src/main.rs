// [server] main.rs
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use server::ThreadPool;

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Starting server...");

    // spin up threadpool
    let pool: ThreadPool = match ThreadPool::build(10) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            return
        }
    };

    // handle incoming TCP requests
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Requested value: {:#?}, fib result: {:#?}", 
            http_request[0].parse::<u128>().unwrap(),
            fibonacci((http_request[0].parse::<u128>()).unwrap()));
}