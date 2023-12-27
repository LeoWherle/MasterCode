use rand;
use rand::seq::SliceRandom;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tracing::{debug, info, error, trace};

use common::{Color, Guess, Response};

fn generate_secret_sequence() -> [Color; 5] {
    let mut rng = rand::thread_rng();

    // Generate a random sequence of 5 colors from the enum
    let secret_sequence: [Color; 5] = [
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(&mut rng)
        .unwrap(),
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(&mut rng)
        .unwrap(),
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(&mut rng)
        .unwrap(),
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(&mut rng)
        .unwrap(),
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(&mut rng)
        .unwrap(),
    ];

    secret_sequence
}

fn check_guess(secret: &[Color; 5], guess: &[Color; 5]) -> Response {
    let mut correct_positions = 0;
    let mut correct_colors = 0;
    let mut secret_copy = secret.to_vec();

    for (index, &color) in guess.iter().enumerate() {
        if color == secret[index] {
            correct_positions += 1;
        }
    }

    // check if the secret sequence contains the 2 times the same color, the guess 
    // shouldn't count it as 2 correct colors
    for color in guess {
        if secret_copy.contains(color) {
            correct_colors += 1;
            secret_copy.remove(secret_copy.iter().position(|&x| x == *color).unwrap());
        }
    }

    // Exclude correct positions from correct colors count
    correct_colors -= correct_positions;

    Response {
        correct_positions,
        correct_colors,
    }
}

fn handle_client(mut stream: TcpStream) {
    let secret_sequence = generate_secret_sequence();

    info!("Secret sequence: {:?}", secret_sequence);
    loop {
        let mut buffer = [0; 20];
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let guessed_colors: Guess = bincode::deserialize(&buffer).unwrap();
                debug!("Received guess: {:?}", guessed_colors.colors);

                let response = check_guess(&secret_sequence, &guessed_colors.colors);
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
