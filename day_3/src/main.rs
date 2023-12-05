use regex::Regex;
use std::fs;

#[derive(Debug)]
enum SchematicSpan<'a> {
    Dots {
        line_number: usize,
        start: usize,
        end: usize,
    },
    Symbols {
        line_number: usize,
        start: usize,
        end: usize,
        value: &'a str,
    },
    PartNumber {
        line_number: usize,
        start: usize,
        end: usize,
        value: usize,
    },
}

fn parse_line(line: &str, line_number: usize) -> Vec<SchematicSpan> {
    let mut spans = Vec::new();

    let dot_pattern = Regex::new("\\.+").expect("hardcoded");
    for raw_span in dot_pattern.find_iter(line) {
        let start = raw_span.start();
        let end = raw_span.end() - 1;
        let span = SchematicSpan::Dots {
            line_number,
            start,
            end,
        };

        spans.push(span);
    }

    let symbol_pattern = Regex::new(r"[^\\.\d]+").expect("hardcoded");
    for raw_span in symbol_pattern.find_iter(line) {
        let start = raw_span.start();
        let end = raw_span.end() - 1;
        let value = raw_span.as_str();
        let span = SchematicSpan::Symbols {
            line_number,
            start,
            end,
            value,
        };

        spans.push(span);
    }

    let digit_pattern = Regex::new(r"\d+").expect("hardcoded");
    for raw_span in digit_pattern.find_iter(line) {
        let start = raw_span.start();
        let end = raw_span.end() - 1;
        let value = raw_span.as_str().parse::<usize>().unwrap();
        let span = SchematicSpan::PartNumber {
            line_number,
            start,
            end,
            value,
        };
        spans.push(span);
    }

    spans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Read file input.txt. Num lines: {}", input.lines().count());
    //     let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    println!("{}", input);

    let mut spans = Vec::new();
    for (line_number, line) in input.lines().enumerate() {
        let line_spans = parse_line(line, line_number);
        spans.extend(line_spans);
    }

    let part_numbers: Vec<&SchematicSpan> = spans
        .iter()
        .filter(|&schematic_span| matches!(schematic_span, SchematicSpan::PartNumber { .. }))
        .collect();

    let symbols = spans.iter().filter(|&schematic_span| {
        matches!(schematic_span, SchematicSpan::Symbols { value: "*", .. })
    });

    let mut sum = 0;
    for gear in symbols {
        if let SchematicSpan::Symbols {
            line_number: gear_line,
            start: gear_start,
            end: gear_end,
            ..
        } = gear
        {
            println!("Gear: {:?}", gear);
            let mut neighbor_parts = part_numbers.iter().filter(|&part| {
                if let SchematicSpan::PartNumber {
                    line_number: part_line,
                    start: part_start,
                    end: part_end,
                    ..
                } = part
                {
                    let above_bound = if *gear_line == 0 { 0 } else { *gear_line - 1 };
                    let below_bound = *gear_line + 1;

                    let within_1_above = above_bound <= *part_line;
                    let within_1_below = below_bound >= *part_line;
                    println!("within_1_above: {}", within_1_above);
                    println!("within_1_below: {}", within_1_below);

                    if within_1_above && within_1_below {
                        let left_bound = if *gear_start == 0 { 0 } else { *gear_start - 1 };
                        let right_bound = *gear_end + 1;

                        let within_1_left = left_bound <= *part_end;
                        let within_1_right = right_bound >= *part_start;
                        println!("within_1_left: {}", within_1_left);
                        println!("within_1_right: {}", within_1_right);

                        within_1_left && within_1_right
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if neighbor_parts.clone().count() == 2 {
                if let SchematicSpan::PartNumber {
                    value: first_neighbor_value,
                    ..
                } = neighbor_parts.next().unwrap()
                {
                    if let SchematicSpan::PartNumber {
                        value: second_neighbor_value,
                        ..
                    } = neighbor_parts.next().unwrap()
                    {
                        println!("{} - {}", first_neighbor_value, second_neighbor_value);
                        sum += first_neighbor_value * second_neighbor_value;
                    }
                }
            }
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}
