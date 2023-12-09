use std::fs;

fn parse(content: &str) -> Vec<Vec<i64>> {
    return content
        .lines()
        .map(|line| {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            return numbers
                .iter()
                .map(|number| number.parse::<i64>().unwrap())
                .collect();
        })
        .collect();
}

fn part1(content: &str) -> i64 {
    let data = parse(content);

    data.iter()
        .map(|line| {
            let mut results = vec![*line.last().unwrap()];
            let mut windows = line.to_vec();

            while windows.iter().any(|&x| x != 0) {
                windows = windows
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();

                if let Some(&last) = windows.last() {
                    results.push(last);
                }
            }

            results
        })
        .flatten()
        .sum()
}

fn part2(content: &str) -> i64 {
    let data: Vec<Vec<i64>> = parse(content);

    let result: Vec<Vec<i64>> = data
        .into_iter()
        .map(|line| {
            let mut prediction = *line.first().unwrap();
            let mut windows = line;
            let mut values: Vec<i64> = vec![];
            values.push(prediction);
            while !windows.iter().all(|&x| x == 0) {
                windows = windows
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();
                values.push(*windows.first().unwrap());
                prediction = *windows.first().unwrap() - prediction;
            }
            values
        })
        .collect();

    return result
        .iter()
        .map(|values| {
            values.iter().rev().fold(0, |acc, &x| x - acc)
        })
        .sum();

}

fn main() {
    let filename = "data/day9/input.txt";
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
            ::read_to_string("data/day9/test1.txt")
            .expect("Should have been able to read the file");

        assert_eq!(part1(&content), 114);
    }

    #[test]
    fn test_2() {
        let content = fs
            ::read_to_string("data/day9/test1.txt")
            .expect("Should have been able to read the file");
        assert_eq!(part2(&content), 2);
    }
}
