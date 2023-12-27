use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tracing::{debug, error, info, trace};

use common::ColorSequence;

fn handle_client(mut stream: TcpStream) {
    let secret_sequence = ColorSequence::random();

    info!("Secret sequence: {:?}", secret_sequence);
    loop {
        let mut buffer = [0; 20];
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let guessed_colors: ColorSequence = bincode::deserialize(&buffer).unwrap();
                debug!("Received guess: {:?}", guessed_colors);

                let response = secret_sequence.check_guess(&guessed_colors);
                debug!("Sending response: {:?}", response);

                let response_bytes = bincode::serialize(&response).unwrap();
                trace!("Sending response bytes: {:?}", response_bytes.len());
                stream.write_all(&response_bytes).unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
