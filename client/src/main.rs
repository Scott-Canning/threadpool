// [client] main.rs
use std::{
    thread,
    env::args,
    io::{prelude::*, self},
    net::TcpStream,
};
use::rand::{
    Rng,
    distributions::Uniform,
};

// establish TCP connection to server
fn connect(value: u128) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    
    let response: String = format!("{}", value);
    stream.write(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

// spawn threads to connect with the server over TCP
fn spawn_connectors(count: u32) {
    let mut handle_vec = Vec::<thread::JoinHandle<()>>::new();

    // create vec of u128 elements
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 10);
    let rand_values: Vec<u128> = (0..count).map(|_| rng.sample(&range)).collect();

    // spawn 'count' threads
    for e in rand_values {
        handle_vec.push(thread::spawn(move || {
            if let Err(error) = connect(e) {
                eprintln!("Failed to establish connection: {}", error);
            };
            println!("Thread with value: {}", e)
        }));
    }
    
    // join thread handles
    for jh in handle_vec.into_iter() {
        jh.join().unwrap();
    }
}


fn main() {
    println!("Starting client...");

    // default connections for first run
    let mut connection_count: u32 = 10;
    
    // take connection_count input as arg value, otherwise use default count
    let args: Vec<String> = args().collect();
    if args.len() > 1 && args[1].parse::<u32>().is_ok() {
        connection_count = args[1].parse().unwrap();
    }

    spawn_connectors(connection_count);
    
    // allow subsequent user inputs
    let mut input: String = String::new();
    loop {
        println!("Please type an integer, or x to escape:");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from std");

        let trimmed: &str = input.trim();
        if trimmed == "x" {
            break;
        }

        match trimmed.parse::<u32>() {
            Ok(count) => spawn_connectors(count),
            Err(..) => println!("Please enter an integer"),
        };
    }
    println!("See you later!");
}
