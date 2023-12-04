use std::cmp::min;
use std::fs::read_to_string;
use std::collections::HashSet;

fn parse(lines: &[String]) -> Vec<i32> {
    lines
        .iter()
        .map(|line| { line.split(":").last().unwrap_or("") })
        .map(|numbers| {
            let number_split = numbers.split("|").collect::<Vec<&str>>();
            (number_split[0], number_split[1])
        })
        .map(|(solution_numbers, numbers)| {
            let solution_numbers: Vec<i32> = solution_numbers
                .split(" ")
                .filter(|number| number.parse::<i32>().is_ok())
                .map(|number| {
                    return number.parse::<i32>().unwrap();
                })
                .collect::<Vec<i32>>();
            let numbers: Vec<i32> = numbers
                .split(" ")
                .filter(|number| number.parse::<i32>().is_ok())
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (solution_numbers, numbers)
        })
        .map(|(solution_numbers, numbers)| {
            let solution_numbers: HashSet<i32> = HashSet::from_iter(solution_numbers);
            let numbers: HashSet<i32> = HashSet::from_iter(numbers);
            (solution_numbers, numbers)
        })
        .map(|(solution_numbers, numbers)| {
            solution_numbers.intersection(&numbers).cloned().collect::<Vec<i32>>()
        })
        .map(|numbers| {
            if numbers.is_empty() {
                return 0;
            }
            return 1 << (numbers.len() - 1);
        })
        .collect::<Vec<i32>>()
}

fn parse2(lines: &[String]) -> Vec<i32> {
    lines
        .iter()
        .map(|line| { line.split(":").last().unwrap_or("") })
        .map(|numbers| {
            let number_split = numbers.split("|").collect::<Vec<&str>>();
            (number_split[0], number_split[1])
        })
        .map(|(solution_numbers, numbers)| {
            let solution_numbers: Vec<i32> = solution_numbers
                .split(" ")
                .filter(|number| number.parse::<i32>().is_ok())
                .map(|number| {
                    return number.parse::<i32>().unwrap();
                })
                .collect::<Vec<i32>>();
            let numbers: Vec<i32> = numbers
                .split(" ")
                .filter(|number| number.parse::<i32>().is_ok())
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (solution_numbers, numbers)
        })
        .map(|(solution_numbers, numbers)| {
            let solution_numbers: HashSet<i32> = HashSet::from_iter(solution_numbers);
            let numbers: HashSet<i32> = HashSet::from_iter(numbers);
            (solution_numbers, numbers)
        })
        .map(|(solution_numbers, numbers)| {
            solution_numbers.intersection(&numbers).cloned().collect::<Vec<i32>>()
        })
        .map(|numbers| {
            if numbers.is_empty() {
                return 0;
            }
            return numbers.len() as i32;
        })
        .collect::<Vec<i32>>()
}
fn part1(lines: &[String]) -> i32 {
    let result = parse(lines);

    return result.iter().sum();
}

fn part2(lines: &[String]) -> i32 {
    let counts = parse2(lines);
    let mut number_cards = vec![1; counts.len()];

    let length = counts.len() as i32;
    for (card_number, count) in counts.iter().enumerate() {
        let max_len = min((card_number as i32) + count, length - 1);
        let card_ids = card_number + 1..=max_len.try_into().unwrap();

        for card_id in card_ids {
            number_cards[card_id] += number_cards[card_number];
        }
    }

    return number_cards.iter().sum();
}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename).map(|file_content| file_content.lines().map(String::from).collect())
}

fn main() {
    let lines: Vec<String> = read_lines("data/day4/input.txt").unwrap();

    let result1 = part1(&lines);
    println!("Part 1 Result: {}", result1);

    let result2 = part2(&lines);
    println!("Part 2 Result: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let lines = read_lines("data/day4/test1.txt").unwrap();
        assert_eq!(part1(&lines), 13);
    }

    #[test]
    fn test_2() {
        let lines = read_lines("data/day4/test1.txt").unwrap();
        assert_eq!(part2(&lines), 30);
    }
}
