use common::{Color, ColorSequence, Response, ALL_FIELDS};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Could not connect to server");

    // 16 comming from the serialized size of Response
    let mut buffer = [0; 17];

    let mut current_guess = ColorSequence::new(
        Color::Red,
        Color::Red,
        Color::Blue,
        Color::Blue,
        Color::Green,
    );

    let mut possibilities = Vec::with_capacity(5 * 5 * 5 * 5 * 5);

    for &c1 in Color::iter() {
        for &c2 in Color::iter() {
            for &c3 in Color::iter() {
                for &c4 in Color::iter() {
                    for &c5 in Color::iter() {
                        possibilities.push(ColorSequence::new(c1, c2, c3, c4, c5));
                    }
                }
            }
        }
    }

    let mut lost = false;

    loop {
        // Send guess to server
        let guess_bytes = bincode::serialize(&current_guess).unwrap();
        stream
            .write_all(&guess_bytes)
            .expect("Failed to send guess to server");

        // Read response from server
        stream
            .read_exact(&mut buffer)
            .expect("Failed to read response from server");
        let response: Response = bincode::deserialize(&buffer).unwrap();

        println!("Guess: {}, Response: {:?}", current_guess, response);

        if response.correct_positions == ALL_FIELDS {
            break;
        }
        if response.lost {
            lost = true;
            break;
        }

        filter_possibilities(&mut possibilities, &current_guess, &response);
        current_guess = ColorSequence::new_from_possible(&mut possibilities);
    }

    if lost {
        println!("Possibilities left: {}", possibilities.len());
        // print all the possibilities
        for possibility in possibilities {
            println!(
                "{}",
                ColorSequence::new(
                    possibility[0],
                    possibility[1],
                    possibility[2],
                    possibility[3],
                    possibility[4]
                )
            );
        }
    }
}

/// based on (Knuth's algorithm) filter the possibilities based on the response
fn filter_possibilities(
    possibilities: &mut Vec<ColorSequence>,
    guess: &ColorSequence,
    response: &Response,
) {
    let mut i = 0;

    while i < possibilities.len() {
        let mut correct_positions = 0;
        let mut correct_colors = 0;
        let mut secret_copy = possibilities[i].to_vec();

        for (index, &color) in guess.iter().enumerate() {
            if color == possibilities[i][index] {
                correct_positions += 1;
            }
        }

        // check if the secret sequence contains the 2 times the same color, the guess
        // shouldn't count it as 2 correct colors
        for color in guess.iter() {
            if secret_copy.contains(color) {
                correct_colors += 1;
                secret_copy.remove(secret_copy.iter().position(|&x| x == *color).unwrap());
            }
        }

        // Exclude correct positions from correct colors count
        correct_colors -= correct_positions;

        if correct_positions != response.correct_positions
            || correct_colors != response.correct_colors
        {
            possibilities.remove(i);
        } else {
            i += 1;
        }
    }
}

