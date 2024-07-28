use std::{thread, net, time::Duration};
use std::io::prelude::*;
use std::io;

struct ConnectionWithClient {
    uid: i32,
    stream: net::TcpStream,
    addr: net::SocketAddr,
}

fn initiate_new_client(connection: ConnectionWithClient) -> i32 {
    let connection_id: String = connection.uid.to_string();
    let _addr: net::SocketAddr = connection.addr;
    let mut stream: &net::TcpStream = &connection.stream;

    let mut pings: i32 = 0;

    println!("Connection UID: {}", connection_id);

    loop {
        if pings >= 5 {
            println!("All pings to connection '{}' have been send & received, closing connection.", connection_id);
            break;
        }

        let write_status = stream.write_all("Ping!\n".as_bytes());
        match write_status {
            Ok(()) => pings += 1,
            Err(ref e) => {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    println!("Connection '{}' closed by client.", connection_id);
                    break;
                } else {
                    println!("Unexpected error: {}", e)
                }
            },
        };

        thread::sleep(Duration::from_millis(1000));
    }


    return 0;
}

fn main() {
    println!("Initialising Server");
    let mut connections_created: i32 = 0;

    let socket_listener = net::TcpListener::bind("0.0.0.0:8000").unwrap();
    
    loop {
        match socket_listener.accept() {
            Ok((sock, addr)) => {
                connections_created += 1;

                thread::spawn(move || {
                    println!("Created thread handling user '{addr:?}'");

                    let connection: ConnectionWithClient = ConnectionWithClient {
                        uid: connections_created,
                        stream: sock,
                        addr: addr
                    };

                    initiate_new_client(connection);
                });
            },
            Err(e) => println!("Couldn't accept client: {e:?}"),
        };
    }
}
