use std::fs::read_to_string;
use std::collections::HashMap;


fn part1(lines: &[String]) -> i32 {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();


    
    return 0
}

fn part2(lines: &[String]) -> i32 {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();


    return 0
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
        assert_eq!(part1(&lines), 4361);
    }

    #[test]
    fn test_2() {
        let lines = read_lines("data/day4/test1.txt").unwrap();
        assert_eq!(part2(&lines), 467835);
    }
}

      