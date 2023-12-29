use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    LightBlue,
}

impl Color {
    pub fn iter() -> Iter<'static, Color> {
        static COLORS: [Color; 6] = [
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ];
        COLORS.iter()
    }

    pub fn random<R>(rng: &mut R) -> Color
    where
        R: Rng + ?Sized,
    {
        *[
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Pink,
            Color::LightBlue,
        ]
        .choose(rng)
        .unwrap()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // corresponding ansi color codes
        let color = match self {
            Color::Red => "\x1b[31m",
            Color::Blue => "\x1b[34m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Pink => "\x1b[35m",
            Color::LightBlue => "\x1b[36m",
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Serialize, Deserialize , Default)]
pub struct Response {
    pub lost: bool,
    pub correct_positions: usize,
    pub correct_colors: usize,
}


pub const MAX_GUESSES: usize = 6;
pub const ALL_FIELDS: usize = 5;

#[derive(Serialize, Deserialize)]
pub struct ColorSequence([Color; ALL_FIELDS]);

impl ColorSequence {
    pub fn new(first: Color, second: Color, third: Color, fourth: Color, fifth: Color) -> Self {
        ColorSequence([first, second, third, fourth, fifth])
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let secret_sequence: [Color; ALL_FIELDS] = [
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
        ];

        ColorSequence(secret_sequence)
    }

    pub fn iter(&self) -> Iter<'_, Color> {
        self.0.iter()
    }

    pub fn new_from_possible(possibilities: &mut Vec<[Color; ALL_FIELDS]>) -> Self {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..possibilities.len());
        ColorSequence {
            0: possibilities[index],
        }
    }

    pub fn check_guess(&self, guess: &ColorSequence) -> Response {
        let mut correct_positions = 0;
        let mut correct_colors = 0;
        let mut secret_copy = self.0.to_vec();

        for (index, &color) in guess.iter().enumerate() {
            if color == self.0[index] {
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

        Response {
            lost: false,
            correct_positions,
            correct_colors,
        }
    }
}

impl std::fmt::Display for ColorSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for color in self.0.iter() {
            s.push_str(&format!("{}■\x1b[0m ", color));
        }

        write!(f, "{}", s)
    }
}
