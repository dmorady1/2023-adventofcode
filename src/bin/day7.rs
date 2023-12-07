use std::{ fs, collections::HashMap };

fn parse(content: &str) -> Vec<(&str, i32)> {
    let lines: Vec<(&str, i32)> = content
        .lines()
        .map(|line| {
            let mut split_lines = line.split_whitespace();
            let cards = split_lines.nth(0).unwrap();
            let number = split_lines.next().unwrap().parse::<i32>().unwrap();

            return (cards, number);
        })
        .collect();

    return lines;
}

fn count_chars(s: &str) -> i32 {
    let mut char_counts = HashMap::new();

    for c in s.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }

    return char_counts.values().max().unwrap() - (char_counts.values().len() as i32);
}

fn part1(content: &str) -> i32 {
    let hands = parse(content);

    let mut hands_alphabetically: Vec<(String, i32)> = hands
        .into_iter()
        .map(|(cards, numbers)| {
            let new_cards = cards
                .chars()
                .map(|c| {
                    match c {
                        'A' => 'Z',
                        'K' => 'Y',
                        'Q' => 'X',
                        'J' => 'W',
                        'T' => 'V',
                        '9' => 'U',
                        '8' => 'T',
                        '7' => 'S',
                        '6' => 'R',
                        '5' => 'Q',
                        '4' => 'P',
                        '3' => 'O',
                        '2' => 'N',
                        _ => c,
                    }
                })
                .collect();

            (new_cards, numbers)
        })
        .collect();

    hands_alphabetically.sort_by(|(a_str, _), (b_str, _)| {
        let count_a = count_chars(a_str);
        let count_b = count_chars(b_str);

        if count_a == count_b {
            a_str.cmp(&b_str)
        } else {
            count_a.cmp(&count_b)
        }
    });

    return hands_alphabetically
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| bid * ((index as i32) + 1))
        .sum();

}

fn count_chars2(s: &str) -> i32 {
    let mut char_counts = HashMap::new();

    for c in s.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }
    if char_counts.values().len() != 1 && char_counts.contains_key(&'J') {

        let j_count = char_counts.remove(&'J').unwrap();

        return char_counts.values().max().unwrap() + j_count - char_counts.values().len() as i32;
    }


    return char_counts.values().max().unwrap() - (char_counts.values().len() as i32);
}

fn convert_to_alphabetically(cards: &str) -> String {
    return cards
        .chars()
        .map(|c| {
            match c {
                'A' => 'Z',
                'K' => 'Y',
                'Q' => 'X',
                'T' => 'V',
                '9' => 'U',
                '8' => 'T',
                '7' => 'S',
                '6' => 'R',
                '5' => 'Q',
                '4' => 'P',
                '3' => 'O',
                '2' => 'N',
                'J' => 'C',
                _ => c,
            }
        })
        .collect::<String>();
}
fn part2(content: &str) -> i32 {
    let mut hands = parse(content);

    hands.sort_by(|(a_str, _), (b_str, _)| {
        let count_a = count_chars2(a_str);
        let count_b = count_chars2(b_str);

        if count_a == count_b {
            let a_alphabetically = convert_to_alphabetically(a_str);
            let b_alphabetically = convert_to_alphabetically(b_str);

            a_alphabetically.cmp(&b_alphabetically)
        } else {
            count_a.cmp(&count_b)
        }
    });

    return hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| bid * ((index as i32) + 1))
        .sum();

}

fn main() {
    let filename = "data/day7/input.txt";
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
            ::read_to_string("data/day7/test1.txt")
            .expect("Should have been able to read the file");

        assert_eq!(part1(&content), 6440);
    }

    #[test]
    fn test_2() {
        let content = fs
            ::read_to_string("data/day7/test1.txt")
            .expect("Should have been able to read the file");
        assert_eq!(part2(&content), 5905);
    }
}
