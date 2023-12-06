use std::{ fs, iter::zip, f32 };

fn parse(content: &str) -> (Vec<f32>, Vec<f32>) {
    let times: Vec<f32> = content
        .lines()
        .next()
        .unwrap() // Handle the case where there's no line
        .split_whitespace()
        .skip(1)
        .map(|number| number.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
        .unwrap();

    let distances: Vec<f32> = content
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|number| number.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
        .unwrap();

    return (times, distances);
}

fn parse2(content: &str) -> (f32, f32) {
    let time: f32 = content
        .lines()
        .next()
        .unwrap() // Handle the case where there's no line
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse::<f32>()
        .unwrap();

    let distance: f32 = content
        .lines()
        .nth(1)
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse::<f32>()
        .unwrap();

    return (time, distance);
}

fn part1(content: &str) -> u32 {
    let (times, distances) = parse(content);

    let number_of_ways: Vec<u32> = zip(times, distances)
        .map(|(time, distance)| {
            let min_time = time / 2.0 - f32::sqrt(time * time - 4.0 * distance) / 2.0;
            let max_time = time / 2.0 + f32::sqrt(time * time - 4.0 * distance) / 2.0;

            let min_time_adjusted = match min_time.floor() == min_time {
                true => min_time + 1.0, // Exclude integer min_time
                false => min_time.ceil(), // Include non-integer min_time
            };

            let max_time_adjusted = match max_time.floor() == max_time {
                true => max_time, // Exclude integer max_time
                false => max_time.floor() + 1.0, // Include non-integer max_time
            };

            return (max_time_adjusted - min_time_adjusted) as u32;
        })
        .collect();

    return number_of_ways
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .unwrap();
}

fn part2(content: &str) -> u32 {
    let (time, distance) = parse2(content);

    let min_time = time / 2.0 - f32::sqrt(time * time - 4.0 * distance) / 2.0;
    let max_time = time / 2.0 + f32::sqrt(time * time - 4.0 * distance) / 2.0;

    let min_time_adjusted = match min_time.floor() == min_time {
        true => min_time + 1.0, // Exclude integer min_time
        false => min_time.ceil(), // Include non-integer min_time
    };

    let max_time_adjusted = match max_time.floor() == max_time {
        true => max_time, // Exclude integer max_time
        false => max_time.floor() + 1.0, // Include non-integer max_time
    };

    return (max_time_adjusted - min_time_adjusted) as u32;
}

fn main() {
    let filename = "data/day6/input.txt";
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
            ::read_to_string("data/day6/test1.txt")
            .expect("Should have been able to read the file");

        assert_eq!(part1(&content), 288);
    }

    #[test]
    fn test_2() {
        let content = fs
            ::read_to_string("data/day6/test1.txt")
            .expect("Should have been able to read the file");
        assert_eq!(part2(&content), 71503);
    }
}
