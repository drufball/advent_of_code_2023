use std::fs;
use std::error::Error;
use regex::Regex;

fn check_color(count: &str, color: &str, threshold: usize) -> Option<bool> {
    let color_pattern = Regex::new(&format!(r"(?<count>\d+) {}", color)).expect("hardcoded reges");
    if let Some(groups) = color_pattern.captures(count) {
        let color_count: usize = groups.name("count").unwrap().as_str().parse().expect("hardcoded");

        if color_count > threshold {
            return Some(false);
        }

        return Some(true);
    }
    
    None
}

fn get_color_count(count: &str, color: &str) -> Option<usize> {
    let color_pattern = Regex::new(&format!(r"(?<count>\d+) {}", color)).expect("hardcoded reges");
    if let Some(groups) = color_pattern.captures(count) {
        let color_count: usize = groups.name("count").unwrap().as_str().parse().expect("hardcoded");

        Some(color_count)
    } else {
        None
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let games = fs::read_to_string("input.txt")?;

    let mut sum = 0;
    for game in games.lines() {
        let game_pattern = Regex::new(r"Game (?P<game_id>\d+): (?P<rounds>.+)")?;
        let groups = game_pattern.captures(game).unwrap();
        
        let id_match = groups.name("game_id").unwrap();
        let game_id: usize = id_match.as_str().parse()?;
        
        let rounds_match = groups.name("rounds").unwrap();
        let rounds: Vec<&str> = rounds_match.as_str().split("; ").collect();

        //let mut red_ok = true;
        //let mut green_ok = true;
        //let mut blue_ok = true;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for round in &rounds {
            let counts: Vec<&str> = round.split(", ").collect();

            for count in &counts {
                if let Some(red_count) = get_color_count(count, "red") {
                    if red_count > min_red {
                        min_red = red_count;
                    }
                }
                if let Some(green_count) = get_color_count(count, "green") {
                    if green_count > min_green {
                        min_green = green_count;
                    }
                }
                if let Some(blue_count) = get_color_count(count, "blue") {
                    if blue_count > min_blue {
                        min_blue = blue_count;
                    }
                }
            }
        }

        sum += min_red * min_green * min_blue;
        //println!("game {}: {:#?}", game_id, rounds);
    }

    println!("{}", sum);
    Ok(())
}
