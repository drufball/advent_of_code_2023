use std::fs;

fn find_digits(s: &str) -> (usize, usize) {
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // find the first occurence of each digit
    let mut first_digit_position = 1000;
    let mut first_digit = 10;
    let mut last_digit_position = 0;
    let mut last_digit = 10; 
    for (i, &digit) in digits.iter().enumerate() {
        match s.find(digit) {
            Some(position) => {
                if position < first_digit_position || first_digit == 10 {
                    first_digit_position = position;
                    first_digit = i;
                }
            }
            None => (),
        }

        match s.rfind(digit) {
            Some(position) => {
                if position > last_digit_position || last_digit == 10 {
                    last_digit_position = position;
                    last_digit = i;
                }
            }
            None => (),
        }
                
    }

    for (i, &word) in words.iter().enumerate() {
        if let Some(position) = s.find(word) {
            if position < first_digit_position {
                first_digit_position = position;
                first_digit = i + 1;
            }
        }

        if let Some(position) = s.rfind(word) {
            if position > last_digit_position {
                last_digit_position = position;
                last_digit = i + 1;
            }
        }
    }

    (first_digit, last_digit)
}

fn main() {
    let calibration_data = fs::read_to_string("calibration_doc.txt").expect("hardcoded file");
    
    let mut sum = 0;
    for line in calibration_data.lines() {
        let (first_digit, last_digit) = find_digits(line);

        let row_val = (first_digit * 10) + last_digit;
        println!("value for {}: {}", line, row_val); 
        sum += row_val 
    }

    println!("{}", sum);
}
