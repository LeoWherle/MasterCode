use common::{Color, ColorSequence, Response, ALL_FIELDS, MAX_GUESSES};

fn main() {
    println!("chose the secret sequence");
    let secret_sequence = ColorSequence::from_input();
    println!("secret_sequence: {}", secret_sequence);

    let mut guesses_left = MAX_GUESSES as i32;

    let mut current_guess = ColorSequence::from_input();

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

    while guesses_left > 0 {
        guesses_left -= 1;
        let response = current_guess.check_guess(&secret_sequence);
        if response.correct_positions == ALL_FIELDS {
            break;
        }
        if response.lost {
            lost = true;
            break;
        }

        filter_possibilities(&mut possibilities, &current_guess, &response);
        println!("Possibilities left: {}", possibilities.len());
        if possibilities.len() < 10 {
            for possibility in &possibilities {
                println!("{}", possibility);
            }
        }
        current_guess = ColorSequence::from_input();
        println!("chosen sequence: {}", current_guess);
    }

    if lost {
        println!("Possibilities left: {}", possibilities.len());
        // print all the possibilities
        for possibility in possibilities {
            println!("{}", possibility);
        }
    } else {
        println!("Possibilities left: {}", possibilities.len());
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
