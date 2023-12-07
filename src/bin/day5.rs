use std::{ fs, ops::Range };

#[derive(Debug, Clone)]
struct Mapping {
    destination: u64,
    start: u64,
    step: u64,
}

fn mapping(input: u64, mapping: &Mapping) -> u64 {
    if (mapping.start..=mapping.start + mapping.step - 1).contains(&input) {
        mapping.destination + (input - mapping.start)
    } else {
        input
    }
}

fn mapping2(input: u64, mapping: &Mapping) -> u64 {

    if input < mapping.destination {
        return input;
    }
    let previous = input - mapping.destination + mapping.start;
    if (mapping.start..=mapping.start + mapping.step - 1).contains(&previous) {
        previous
    } else {
        input
    }
}

fn parse(content: &str) -> Vec<Vec<Mapping>> {
    content
        .split("\n\n")
        .skip(1)
        .map(|mappings| {
            mappings
                .split('\n')
                .skip(1)
                .filter_map(|line| {
                    let numbers: Vec<u64> = line
                        .split_ascii_whitespace()
                        .filter_map(|number| number.parse::<u64>().ok())
                        .collect();
                    if numbers.len() == 3 {
                        Some(Mapping {
                            destination: numbers[0],
                            start: numbers[1],
                            step: numbers[2],
                        })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn part1(content: &str) -> u64 {
    let input = content
        .lines()
        .next()
        .expect("Content should have at least one line")
        .split_whitespace()
        .skip(1)
        .map(|number| number.parse::<u64>().expect("Parsing error"))
        .collect::<Vec<u64>>();

    let mappings = parse(content);

    let result = input
        .iter()
        .map(|&seed| {
            let mut num = seed;
            for maps in &mappings {
                for map in maps {
                    let old = num;
                    num = mapping(num, map);
                    if num != old {
                        break;
                    }
                }
            }
            num
        })
        .min()
        .expect("There should be a minimum value");

    result
}

fn part2(content: &str) -> u64 {
    let input: Vec<u64> = content
        .lines()
        .next()
        .expect("Content should have at least one line")
        .split_whitespace()
        .skip(1)
        .map(|number| number.parse::<u64>().expect("Parsing error"))
        .collect();
    let mappings: Vec<Vec<Mapping>> = parse(content);

    let new_input: Vec<Range<u64>> = input
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();


    let minimum = 35;

    let maximum = *input.iter().max().unwrap();
    let seeds = minimum..=maximum;

    let mut reverse_mappings = mappings.clone();
    reverse_mappings.reverse();

    for seed in seeds {
        let mut num = seed;
        for maps in &reverse_mappings {
            for map in maps {
                let old = num;
                num = mapping2(num, map);
                if num != old {
                    break;
                }
            }
        }

        if new_input.iter().any(|range| range.contains(&num)) {
            return seed; 
        }
    }

    panic!("No valid num found within the given ranges")
}


fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Should have been able to read the file");
}

fn main() {
    let content = read_file("data/day5/input.txt");

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
        let content = read_file("data/day5/test1.txt");

        assert_eq!(part1(&content), 35);
    }

    #[test]
    fn test_2() {
        let content = read_file("data/day5/test1.txt");
        assert_eq!(part2(&content), 46);
    }
}
