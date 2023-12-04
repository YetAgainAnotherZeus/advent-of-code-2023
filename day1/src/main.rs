#![allow(dead_code, unused_variables)]

struct Digits {
    first: Option<String>,
    last: Option<String>,
}

impl Digits {
    fn new() -> Digits {
        Digits {
            first: None,
            last: None
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let string_slice = input.as_str();

    let result_1 = part1(string_slice);

    println!("Part 1: {}", result_1);
}

fn part1(input: &str) -> i32 {
    let mut calibration_values: Vec<Digits> = Vec::new();

    let cleaned_input = input.replace("\r\n", "\n");
    let lines = cleaned_input.split('\n');

    lines.for_each(|line| {
        let mut current_digits: Digits = Digits::new();

        line.chars().for_each(|c| {
            if c.is_ascii_digit() {
                if current_digits.first.is_none() {
                    current_digits.first = Some(c.to_string());
                }
                current_digits.last = Some(c.to_string());
            }
        });

        calibration_values.push(current_digits);
    });

    let formated_values: Vec<i32> = calibration_values.iter().map(|digits| {
        let num = format!("{}{}", digits.first.clone().unwrap(), digits.last.clone().unwrap());

        num.parse::<i32>().unwrap()
    }).collect();

    formated_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part_1() {
        let test_string = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(part1(test_string), 142);
    }
}