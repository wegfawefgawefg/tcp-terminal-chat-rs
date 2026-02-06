use std::{
    io::Read,
    io::{self, ErrorKind, Write},
    net::{TcpListener, TcpStream},
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const ADDRESS: &str = "127.0.0.1:7878";

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> io::Result<()> {
    let mut buffer = [0; 32];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {:?}", e);
                return Err(e);
            }
        };

        let msg = match String::from_utf8(buffer[..bytes_read].to_vec()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode message: {:?}", e);
                continue;
            }
        };

        let mut clients = clients.lock().unwrap();
        let mut i = 0;
        while i != clients.len() {
            if let Err(e) = clients[i]
                .write_all(msg.as_bytes())
                .and_then(|_| clients[i].flush())
            {
                eprintln!("Failed to send message: {:?} removing client", e);
                clients.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    let mut listener = None;
    let mut attempts = 0;

    while listener.is_none() && attempts < 5 {
        match TcpListener::bind(ADDRESS) {
            Ok(l) => listener = Some(l),
            Err(e) => {
                eprintln!("Failed to bind: {:?}", e);
                thread::sleep(Duration::from_secs(5));
                attempts += 1;
            }
        }
    }

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    if let Some(listener) = &mut listener {
        for stream_res in listener.incoming() {
            match stream_res {
                Ok(stream) => {
                    println!("Connection established!");
                    match stream.try_clone() {
                        Ok(stream_clone) => {
                            let clients = Arc::clone(&clients);
                            clients.lock().unwrap().push(stream_clone);
                            thread::spawn(move || {
                                handle_client(stream, clients)
                                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                            });
                        }
                        Err(e) => eprintln!("error: {:?}", e),
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::ConnectionRefused
                    | ErrorKind::ConnectionReset
                    | ErrorKind::ConnectionAborted => {
                        eprintln!("Connection error: {:?}", e);
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                    _ => {
                        eprintln!("Application error: {:?}", e);
                        process::exit(1);
                    }
                },
            }
        }
    }
}

