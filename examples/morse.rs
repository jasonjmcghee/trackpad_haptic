use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use trackpad_haptic::feedback::{Feedback, FeedbackManager};

fn main() {
    loop {
        sleep(Duration::from_secs(1));
        trigger_haptic_feedback();
    }
}

#[derive(Debug)]
pub enum Morse {
    Dot,
    Dash,
    Space,
}

impl Morse {
    pub fn get_millis(&self) -> u64 {
        match self {
            Morse::Dot => 100,
            Morse::Dash => 1000,
            Morse::Space => 1000,
        }
    }

    fn from_char(c: char) -> Vec<Morse> {
        match c.to_ascii_uppercase() {
            'A' => vec![Morse::Dot, Morse::Dash],
            'B' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot],
            'C' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot],
            'D' => vec![Morse::Dash, Morse::Dot, Morse::Dot],
            'E' => vec![Morse::Dot],
            'F' => vec![Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dot],
            'G' => vec![Morse::Dash, Morse::Dash, Morse::Dot],
            'H' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot],
            'I' => vec![Morse::Dot, Morse::Dot],
            'J' => vec![Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dash],
            'K' => vec![Morse::Dash, Morse::Dot, Morse::Dash],
            'L' => vec![Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dot],
            'M' => vec![Morse::Dash, Morse::Dash],
            'N' => vec![Morse::Dash, Morse::Dot],
            'O' => vec![Morse::Dash, Morse::Dash, Morse::Dash],
            'P' => vec![Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot],
            'Q' => vec![Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dash],
            'R' => vec![Morse::Dot, Morse::Dash, Morse::Dot],
            'S' => vec![Morse::Dot, Morse::Dot, Morse::Dot],
            'T' => vec![Morse::Dash],
            'U' => vec![Morse::Dot, Morse::Dot, Morse::Dash],
            'V' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash],
            'W' => vec![Morse::Dot, Morse::Dash, Morse::Dash],
            'X' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dash],
            'Y' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dash],
            'Z' => vec![Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot],
            '1' => vec![Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dash],
            '2' => vec![Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dash],
            '3' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dash],
            '4' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash],
            '5' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot],
            '6' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot],
            '7' => vec![Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot],
            '8' => vec![Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot],
            '9' => vec![Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dot],
            '0' => vec![Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dash],
            ',' => vec![Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dash],
            '.' => vec![Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dash],
            '?' => vec![Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot],
            '\'' => vec![Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dot],
            '!' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dash],
            '/' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dot],
            '(' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot],
            ')' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dash],
            '&' => vec![Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot],
            ':' => vec![Morse::Dash, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot],
            ';' => vec![Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot],
            '=' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash],
            '+' => vec![Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot],
            '-' => vec![Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash],
            '_' => vec![Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dash],
            '"' => vec![Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dot],
            '$' => vec![Morse::Dot, Morse::Dot, Morse::Dot, Morse::Dash, Morse::Dot, Morse::Dot, Morse::Dash],
            '@' => vec![Morse::Dot, Morse::Dash, Morse::Dash, Morse::Dot, Morse::Dash, Morse::Dot],

            ' ' => vec![Morse::Space],
            _ => vec![],
        }
    }
}

impl Display for Morse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Morse::Dot => write!(f, "."),
            Morse::Dash => write!(f, "-"),
            Morse::Space => write!(f, " "),
        }
    }
}

pub fn english_to_morse(input: &str) -> Vec<Morse> {
    input.chars().flat_map(Morse::from_char).collect()
}


trait Player<T> {
    fn play(&self, subject: T);
}

impl Player<Morse> for FeedbackManager {
    fn play(&self, symbol: Morse) {
        println!("{}", symbol);
        let delay: u64 = 200;
        match symbol {
            Morse::Space => {
                sleep(Duration::from_millis(symbol.get_millis() + delay))
            }

            Morse::Dot => {
                self.trigger_with_feedback(Feedback::new(
                    symbol.get_millis(), delay
                ));
            }
            Morse::Dash => {
                self.trigger_with_feedback(Feedback::new(
                    symbol.get_millis(), delay * 2
                ));
            }
        }
    }
}

fn trigger_haptic_feedback() {
    // Get the default haptic feedback manager
    let haptic_manager = FeedbackManager::default();

    for symbol in english_to_morse("SOS") {
        haptic_manager.play(symbol)
    }
}
