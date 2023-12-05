use regex::Regex;
use std::error::Error;
use std::fs;

fn get_input() -> String {
    let input = fs::read_to_string("input.txt").expect("hardcoded file");
    println!("Read file! Lines: {}", input.lines().count());

    // let input = String::from("\
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    // println!("Using test input");

    input
}
fn split_line(input: &str) -> Vec<&str> {
    let delimeters = [':', '|'];
    input.split(|c| delimeters.contains(&c)).collect()
}

fn parse_numbers(raw_numbers: &str) -> Vec<usize> {
    let number_pattern = Regex::new(r"(\d+)").expect("hardcoded regex");

    number_pattern
        .find_iter(raw_numbers)
        .map(|raw_number| raw_number.as_str().parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    player_numbers: Vec<usize>,
}

impl Card {
    fn build(raw_card_values: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        let id = parse_numbers(raw_card_values[0])[0];
        let winning_numbers = parse_numbers(raw_card_values[1]);
        let player_numbers = parse_numbers(raw_card_values[2]);

        Ok(Self {
            id,
            winning_numbers,
            player_numbers,
        })
    }

    fn correct_numbers(&self) -> Vec<usize> {
        let mut correct_numbers = Vec::new();
        for number in self.player_numbers.iter() {
            if self.winning_numbers.contains(number) {
                correct_numbers.push(*number);
            }
        }

        correct_numbers
    }

    // fn score(&self) -> usize {
    //     if self.correct_numbers().is_empty() {
    //         0
    //     } else {
    //         1 << (self.correct_numbers().len() - 1)
    //     }
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input();

    // let mut points = 0;
    let mut copies: Vec<usize> = vec![1; input.lines().count()];
    for line in input.lines() {
        let card = Card::build(split_line(line))?;

        // Card indexes are 1-indexed, making an itermediate variable
        // here for clarity
        let current_card_index = card.id - 1;

        let num_correct = card.correct_numbers().len();
        // NOTE: inclusive range
        for i in 1..=num_correct {
            copies[current_card_index + i] += copies[current_card_index];
        }
        // points += card.score();
    }

    println!("copies: {:?}", copies);
    let num_cards: usize = copies.iter().sum();
    println!("{:?}", num_cards);
    // println!("{}", points);

    Ok(())
}
