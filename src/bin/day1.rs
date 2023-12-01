use regex::Regex;
use std::fs::read_to_string;
use std::collections::HashMap;

fn part1(lines: &Vec<String>) -> u32 {
    let re2digits = Regex::new(r"^.*?(\d).*(\d).*$").unwrap();
    let re1digit = Regex::new(r"^.*(\d).*$").unwrap();
    let result: u32 = lines
        .iter()
        .map(|line| {
            let caps2digits = re2digits.captures(line);

            if let Some(caps) = caps2digits {
                let two_digits = format!(
                    "{}{}",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str()
                );
                return two_digits.parse::<u32>().unwrap_or(0);
            }

            let caps1digit = re1digit.captures(line);
            if let Some(caps) = caps1digit {
                let two_digits = format!(
                    "{}{}",
                    caps.get(1).unwrap().as_str(),
                    caps.get(1).unwrap().as_str()
                );
                return two_digits.parse::<u32>().unwrap_or(0);
            }

            return 0
        })
        .sum();

    return result;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut scores:HashMap<&str, &str> = HashMap::new();
    scores.insert("one", "1");
    scores.insert("two", "2");
    scores.insert("three","3");
    scores.insert("four", "4");
    scores.insert("five", "5");
    scores.insert("six", "6");
    scores.insert("seven", "7");
    scores.insert("eight", "8");
    scores.insert("nine", "9");


    let re2digits = Regex::new(r"^.*?(one|two|three|four|five|six|seven|eight|nine|\d).*(one|two|three|four|five|six|seven|eight|nine|\d).*$").unwrap();
    let re1digit = Regex::new(r"^.*(one|two|three|four|five|six|seven|eight|nine|\d).*$").unwrap();
    let result: u32 = lines
        .iter()
        .map(|line| {
            let caps2digits = re2digits.captures(line);

            if let Some(caps) = caps2digits {
                let number1 =caps.get(1).unwrap().as_str();
                let number2 =caps.get(2).unwrap().as_str();

                let number1 = match scores.get(number1) {
                    Some(number1) => number1,
                    None => number1
                };
                let number2 = match scores.get(number2) {
                    Some(number2) => number2,
                    None => number2
                };


                let two_digits = format!(
                    "{}{}",
                    number1,
                    number2
                );
                return two_digits.parse::<u32>().unwrap_or(0);
            }

            let caps1digit = re1digit.captures(line);
            if let Some(caps) = caps1digit {
                let number1 =caps.get(1).unwrap().as_str();

                let two_digits = format!(
                    "{}{}",
                    number1,
                    number1
                );
                return two_digits.parse::<u32>().unwrap_or(0);
            }

            return 0
        })
        .sum();

    return result;
}

fn main() {
    let lines = read_lines("data/day1/input1.txt");

    let result1 = part1(&lines);
    println!("{}", result1);


    let result2 = part2(&lines);
    println!("{}", result2);

}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
