use std::io::{Read, Write};
use std::net::TcpStream;
use common::{Color, Guess, Response};
use rand::Rng;


fn create_initial_guess() -> Guess {
    // Initial guess as per Knuth's algorithm (11223)
    Guess {
        colors: [Color::Red, Color::Red, Color::Blue, Color::Blue, Color::Green],
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Could not connect to server");
    let mut buffer = [0; 16];
    let mut guesses_left = 6;
    let mut won = false;

    let mut current_guess = create_initial_guess();

    let mut possibilities = vec![];

    for &c1 in Color::iter() {
        for &c2 in Color::iter() {
            for &c3 in Color::iter() {
                for &c4 in Color::iter() {
                    for &c5 in Color::iter() {
                        possibilities.push([c1, c2, c3, c4, c5]);
                    }
                }
            }
        }
    }

    while guesses_left >= 0 {

        let guess_bytes = bincode::serialize(&current_guess).unwrap();
        println!("Sending guess of size: {}", guess_bytes.len());
        stream.write_all(&guess_bytes).expect("Failed to send guess to server");

        // Read response from server
        stream.read_exact(&mut buffer).expect("Failed to read response from server");
        let response: Response = bincode::deserialize(&buffer).unwrap();

        println!("Guess: {:?}, Response: {:?}", current_guess.colors, response);

        if response.correct_positions == 5 {
            won = true;
            break;
        }
        // Modify current guess based on received feedback (Knuth's algorithm)
        filter_possibilities(&mut possibilities, current_guess, response);
        current_guess = new_guess(&mut possibilities);
        guesses_left -= 1;
    }
    if won == true {
        println!("GG, secret sequence guessed! in {} guesses", 6 - guesses_left);
    } else {
        println!("Game over, what a noob!");
    }
}

fn filter_possibilities(possibilities: &mut Vec<[Color; 5]>, guess: Guess, response: Response) {
    let mut i = 0;

    while i < possibilities.len() {
        let mut correct_positions = 0;
        let mut correct_colors = 0;
        let mut secret_copy = possibilities[i].to_vec();

        for (index, &color) in guess.colors.iter().enumerate() {
            if color == possibilities[i][index] {
                correct_positions += 1;
            }
        }

        // check if the secret sequence contains the 2 times the same color, the guess 
        // shouldn't count it as 2 correct colors
        for color in guess.colors.iter() {
            if secret_copy.contains(color) {
                correct_colors += 1;
                secret_copy.remove(secret_copy.iter().position(|&x| x == *color).unwrap());
            }
        }

        // Exclude correct positions from correct colors count
        correct_colors -= correct_positions;

        if correct_positions != response.correct_positions || correct_colors != response.correct_colors {
            possibilities.remove(i);
        } else {
            i += 1;
        }
    }
}

fn new_guess(possibilities: &mut Vec<[Color; 5]>) -> Guess {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possibilities.len());
    Guess {
        colors: possibilities[index],
    }
}
