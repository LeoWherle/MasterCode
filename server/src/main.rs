use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tracing::{debug, error, info, trace};
use common::{ColorSequence, ALL_FIELDS, MAX_GUESSES, Response};

fn handle_client(mut stream: TcpStream) {
    let secret_sequence = ColorSequence::random();
    let mut guesses_left = MAX_GUESSES as i32;
    let mut won = false;

    info!("Secret sequence: {}", secret_sequence);
    while guesses_left > 0 {
        let mut buffer = [0; 20];
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                guesses_left -= 1;
                let guessed_colors: ColorSequence = bincode::deserialize(&buffer).unwrap();
                debug!("Received guess: {}", guessed_colors);

                let mut response = secret_sequence.check_guess(&guessed_colors);
                if response.correct_positions == ALL_FIELDS {
                    won = true;
                    break;
                }
                if guesses_left == 0 {
                    response.lost = true;
                    break;
                }
                trace!("Sending response: {:?}", response);

                let response_bytes = bincode::serialize(&response).unwrap();
                stream.write_all(&response_bytes).unwrap();
            }
            Err(_) => break,
        }
    }
    let mut final_response = Response {
        lost: false,
        correct_positions: 0,
        correct_colors: 0,
    };
    if !won {
        final_response.lost = true;
        info!("Client lost! Secret sequence was {}", secret_sequence);
    } else {
        final_response.correct_positions = ALL_FIELDS;
        info!("Client won! in {} guesses", 6 - guesses_left);
    }
    let final_response_bytes = bincode::serialize(&final_response).unwrap();
    stream.write_all(&final_response_bytes).unwrap();
}

fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
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
