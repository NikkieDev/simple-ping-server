use dotenv::dotenv;
use std::thread;

pub mod network;

fn main() {
    println!("Starting server");
    dotenv().ok(); // initialize env variables;

    let server_port: &str = &std::env::var("SERVER_PORT").expect("Server Port must be set");
    
    let listener = network::start_listen(server_port);

    let t_accept_connections: thread::JoinHandle<_> = thread::spawn(move || {
        println!("Spanwed connection accepting thread");
        
        loop {
            match listener.accept() {
                Ok((_sock, addr)) => {
                    println!("Connection from '{}'", addr);
                },
                Err(_e) => println!("Couldn't accept client"),
            }
        }
    });

    t_accept_connections.join().expect("The connection acception thread has panicked!");
}