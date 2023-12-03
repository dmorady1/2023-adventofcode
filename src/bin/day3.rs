use std::fs::read_to_string;
use std::collections::HashMap;


fn valid_numbers(grid: Vec<Vec<char>>) -> Vec<i32> {
    let mut numbers: HashMap<(i32, i32), char> = HashMap::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (column, character) in line.iter().enumerate() {
            match character {
                '0'..='9' => numbers.insert((row as i32, column as i32), character.clone()),
                '.' => None,
                _ => symbols.insert((row as i32, column as i32), character.clone()),
            };
        }
    }

    let result: Vec<i32> = symbols.keys().flat_map(|(row, column)| {
        let positions = [
            (row - 1, column),
            (row + 1, column),
            (row.clone(), &(column - 1)),
            (row.clone(), &(column + 1)),
            (row + 1, &(column - 1)),
            (row + 1, &(column + 1)),
            (row - 1, &(column - 1)),
            (row - 1, &(column + 1)),
        ];

        let mut valid_numbers = Vec::new();

        for &(r, c) in &positions {
            let number = get_number(&mut numbers, &r, &c);
            if number != -1 {
                valid_numbers.push(number);
            }
        }

        valid_numbers

    }).collect();


    return result;
}

fn valid_numbers2(grid: Vec<Vec<char>>) -> Vec<i32> {
    let mut numbers: HashMap<(i32, i32), char> = HashMap::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (column, character) in line.iter().enumerate() {
            match character {
                '0'..='9' => numbers.insert((row as i32, column as i32), character.clone()),
                '*' => symbols.insert((row as i32, column as i32), character.clone()),
                '.' | _ => None,
            };
        }
    }

    let result: Vec<i32> = symbols.keys().flat_map(|(row, column)| {
        let positions = [
            (row - 1, column),
            (row + 1, column),
            (row.clone(), &(column - 1)),
            (row.clone(), &(column + 1)),
            (row + 1, &(column - 1)),
            (row + 1, &(column + 1)),
            (row - 1, &(column - 1)),
            (row - 1, &(column + 1)),
        ];

        let mut valid_numbers = Vec::new();

        for &(r, c) in &positions {
            let number = get_number(&mut numbers, &r, &c);
            if number != -1 {
                valid_numbers.push(number);
            }
        }

        if valid_numbers.len() == 2 {
            return  vec![valid_numbers[0] * valid_numbers[1]];
        }
        return vec![];

    }).collect();


    return result;
}
fn get_number(numbers: &mut HashMap<(i32, i32), char>, row: &i32, column: &i32) -> i32 {
    if let Some(&middle) = numbers.get(&(*row, *column)) {
        numbers.remove(&(*row, *column));
        let mut right = String::new();
        let mut y = column + 1;
        while let Some(&num) = numbers.get(&(*row, y)) {
            right.push(num);
            numbers.remove(&(*row, y));
            y += 1;
        }

        let mut left = String::new();
        let mut y = column - 1;
        while let Some(&num) = numbers.get(&(*row, y)) {
            left.push(num);
            numbers.remove(&(*row, y)); 
            y -= 1;
        }

        left = left.chars().rev().collect();
        left.push(middle);

        return left.chars().chain(right.chars()).collect::<String>().parse::<i32>().unwrap();
    }

    -1
}

fn part1(lines: &[String]) -> i32 {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();


    let result = valid_numbers(grid);

    
    return result.iter().sum();
}

fn part2(lines: &[String]) -> i32 {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();


    let result = valid_numbers2(grid);

    
    return result.iter().sum();
}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename).map(|file_content| file_content.lines().map(String::from).collect())
}

fn main() {
    let lines: Vec<String> = read_lines("data/day3/input.txt").unwrap();

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
        let lines = read_lines("data/day3/test1.txt").unwrap();
        assert_eq!(part1(&lines), 4361);
    }

    #[test]
    fn test_2() {
        let lines = read_lines("data/day3/test1.txt").unwrap();
        assert_eq!(part2(&lines), 467835);
    }
}