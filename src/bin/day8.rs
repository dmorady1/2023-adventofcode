use std::{collections::HashMap, fs};

fn parse(content: &str) -> (&str, HashMap<&str, Vec<String>>) {
    let data: Vec<&str> = content.split("\n\n").collect();

    let instructions = data[0];

    let mapping: HashMap<_, _> = data[1]
        .lines()
        .map(|line| {
            let splitted_line: Vec<&str> = line.split(" = ").collect();
            let key = splitted_line[0];
            let values: Vec<String> = splitted_line[1]
                .replace("(", "")
                .replace(")", "")
                .split(", ")
                .map(|s| s.to_string())
                .collect();

            return (key, values);
        })
        .collect();

    return (instructions, mapping);
}

fn part1(content: &str) -> usize {
    let (instructions, mapping) = parse(content);

    let index_instructions: Vec<u64> = instructions
        .chars()
        .into_iter()
        .map(|character| {
            if character == 'L' {
                return 0;
            }
            return 1;
        })
        .collect();
    let mut counter = 0;

    let mut position = "AAA";

    let length = index_instructions.len();
    while position != "ZZZ" {
        let moves: &Vec<String> = mapping.get(position).unwrap();
        let index: usize = counter % length;

        let left_right: u64 = index_instructions[index];
        position = &moves[left_right as usize];

        counter += 1;
    }

    return counter;
}

fn ending_and_cycle(
    position: &str,
    index_instructions: &Vec<u64>,
    length: usize,
    mapping: &HashMap<&str, Vec<String>>
) -> u64 {
    let mut seen: HashMap<&str, u64> = HashMap::new();
    let mut ends: HashMap<&str, u64> = HashMap::new();

    let mut counter: usize = 0;

    let mut new_position = position;
    while !seen.contains_key(new_position) || ends.is_empty() || counter < length {
        seen.insert(new_position, counter as u64);

        let moves: &Vec<String> = mapping.get(new_position).unwrap();
        let index: usize = counter % length;

        let left_right: u64 = index_instructions[index];
        new_position = &moves[left_right as usize];

        counter += 1;
        if new_position.ends_with('Z') && !ends.contains_key(new_position) {
            ends.insert(new_position, counter as u64);
        }
    }

    return ends.values().next().unwrap().clone();
}


fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}


fn lcm_of_vector(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

fn part2(content: &str) -> u64 {
    let (instructions, mapping) = parse(content);

    let index_instructions: Vec<u64> = instructions
        .chars()
        .map(|character| if character == 'L' { 0 } else { 1 })
        .collect();

    let positions: Vec<&str> = mapping
        .keys()
        .filter(|position| position.ends_with('A'))
        .map(|&position| position) // Dereference here
        .collect();

    let length = index_instructions.len();

    for position in positions.iter() {
        ending_and_cycle(position, &index_instructions, length.clone(), &mapping);
    }


    let end_counts: Vec<u64> = positions
    .iter()
    .map(|position| ending_and_cycle(position, &index_instructions, length.clone(), &mapping))
    .collect();




    return lcm_of_vector(&end_counts);
}

fn main() {
    let filename = "data/day8/input.txt";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");

    let result1 = part1(&content);
    println!("Part 1 Result: {}", result1);

    let result2 = part2(&content);
    println!("Part 2 Result: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let content = fs
            ::read_to_string("data/day8/test1.txt")
            .expect("Should have been able to read the file");

        assert_eq!(part1(&content), 6);
    }

    #[test]
    fn test_2() {
        let content = fs
            ::read_to_string("data/day8/test2.txt")
            .expect("Should have been able to read the file");
        assert_eq!(part2(&content), 6);
    }
}
