use std::net;

pub fn start_listen(port: &str) -> net::TcpListener {
    let connect_string: String = format!("0.0.0.0:{}", port);
    let listener = net::TcpListener::bind(connect_string).expect("Unable to bind, is port {port} free?");
    
    println!("Listening on port {}", port);
    listener
}