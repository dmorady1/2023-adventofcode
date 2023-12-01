use regex::Regex;
use std::fs::read_to_string;
use std::collections::HashMap;

fn extract_digits(line: &str, re2digits: &Regex, re1digit: &Regex, scores: &HashMap<&str, &str>) -> u32 {
    if let Some(caps) = re2digits.captures(line) {
        let number1 = caps.get(1).unwrap().as_str();
        let number2 = caps.get(2).unwrap().as_str();

        let number1 = *scores.get(number1).unwrap_or(&number1);
        let number2 = *scores.get(number2).unwrap_or(&number2);

        return format!("{}{}", number1, number2).parse::<u32>().unwrap_or(0);
    }

    if let Some(caps) = re1digit.captures(line) {
        let number1 = caps.get(1).unwrap().as_str();
        let number1 = *scores.get(number1).unwrap_or(&number1);

        return format!("{}{}", number1, number1).parse::<u32>().unwrap_or(0);
    }

    0
}

fn part1(lines: &[String]) -> u32 {
    let re2digits = Regex::new(r"^.*?(\d).*(\d).*$").unwrap();
    let re1digit = Regex::new(r"^.*(\d).*$").unwrap();
    let scores = HashMap::new();

    lines.iter().map(|line| extract_digits(line, &re2digits, &re1digit, &scores)).sum()
}

fn part2(lines: &[String]) -> u32 {
    let mut scores: HashMap<&str, &str> = HashMap::new();
    scores.insert("one", "1");
    scores.insert("two", "2");
    scores.insert("three", "3");
    scores.insert("four", "4");
    scores.insert("five", "5");
    scores.insert("six", "6");
    scores.insert("seven", "7");
    scores.insert("eight", "8");
    scores.insert("nine", "9");

    let re2digits = Regex::new(r"^.*?(one|two|three|four|five|six|seven|eight|nine|\d).*(one|two|three|four|five|six|seven|eight|nine|\d).*$").unwrap();
    let re1digit = Regex::new(r"^.*(one|two|three|four|five|six|seven|eight|nine|\d).*$").unwrap();

    lines.iter().map(|line| extract_digits(line, &re2digits, &re1digit, &scores)).sum()
}

fn main() {
    let lines = read_lines("data/day1/input1.txt").unwrap();

    let result1 = part1(&lines);
    println!("Part 1 Result: {}", result1);

    let result2 = part2(&lines);
    println!("Part 2 Result: {}", result2);
}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename)
        .map(|file_content| file_content.lines().map(String::from).collect())
}
