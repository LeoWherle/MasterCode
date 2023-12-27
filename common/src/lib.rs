use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Guess {
    pub colors: [Color; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub correct_positions: usize,
    pub correct_colors: usize,
}

pub struct ColorSequence([Color; 5]);

impl ColorSequence {
    pub fn new(first: Color, second: Color, third: Color, fourth: Color, fifth: Color) -> Self {
        ColorSequence([first, second, third, fourth, fifth])
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let secret_sequence: [Color; 5] = [
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
            Color::random(&mut rng),
        ];

        ColorSequence(secret_sequence)
    }

    pub fn check_guess(&self, guess: &Guess) -> Response {
        let mut correct_positions = 0;
        let mut correct_colors = 0;
        let mut secret_copy = self.0.to_vec();

        for (index, &color) in guess.colors.iter().enumerate() {
            if color == self.0[index] {
                correct_positions += 1;
                secret_copy[index] = Color::LightBlue;
            }
        }

        for color in guess.colors.iter() {
            if secret_copy.contains(color) {
                correct_colors += 1;
            }
        }

        Response {
            correct_positions,
            correct_colors,
        }
    }
}

// implement debug and display for ColorSequence
impl std::fmt::Debug for ColorSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for color in self.0.iter() {
            s.push_str(&format!("{:?} ", color));
        }

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for ColorSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for color in self.0.iter() {
            s.push_str(&format!("{:?} ", color));
        }

        write!(f, "{}", s)
    }
}

