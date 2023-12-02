use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    game_number: u32,
    color_details: Vec<HashMap<String, u32>>,
}

fn parse(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_number = parts[0].replace("Game ", "").parse::<u32>().unwrap();
    let details_str = parts[1].split("; ");

    let mut color_details = Vec::new();
    for detail in details_str {
        let items = detail.split(", ");
        let mut color_map = HashMap::new();
        for item in items {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1].to_string();
            color_map.insert(color, count);
        }
        color_details.push(color_map);
    }

    Game {
        game_number,
        color_details,
    }
}

fn part1(lines: &[String]) -> i32 {
    let mut values: HashMap<&str, u32> = HashMap::new();
    values.insert("red", 12);
    values.insert("green", 13);
    values.insert("blue", 14);

    let games: Vec<Game> = lines
        .iter()
        .map(|line| parse(line))
        .collect();

    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|game| {
            game.color_details
                .iter()
                .all(|color_detail| {
                    color_detail
                        .iter()
                        .all(|(color, &count)| {
                            values
                                .get(&color.as_str())
                                .map_or(false, |&max_count| count <= max_count)
                        })
                })
        })
        .collect();

    return possible_games
        .iter()
        .map(|possible_game| possible_game.game_number as i32)
        .sum();
}

fn part2(lines: &[String]) -> i32 {
    let mut values: HashMap<&str, u32> = HashMap::new();
    values.insert("red", 12);
    values.insert("green", 13);
    values.insert("blue", 14);

    let games: Vec<Game> = lines
        .iter()
        .map(|line| parse(line))
        .collect();

    return games
        .iter()
        .map(|game| {
            let blue = game.color_details
                .iter()
                .filter_map(|color_detail| color_detail.get("blue").cloned())
                .max()
                .unwrap_or(1);
            let red = game.color_details
                .iter()
                .filter_map(|color_detail| color_detail.get("red").cloned())
                .max()
                .unwrap_or(1);
            let green = game.color_details
                .iter()
                .filter_map(|color_detail| color_detail.get("green").cloned())
                .max()
                .unwrap_or(1);

            (blue * red * green) as i32
        })
        .sum();

}

fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    read_to_string(filename).map(|file_content| file_content.lines().map(String::from).collect())
}

fn main() {
    let lines: Vec<String> = read_lines("data/day2/input.txt").unwrap();

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
        let lines = read_lines("data/day2/test1.txt").unwrap();
        assert_eq!(part1(&lines), 8);
    }
    

    #[test]
    fn test_2() {
        let lines = read_lines("data/day2/test1.txt").unwrap();
        assert_eq!(part2(&lines), 2286);
    }
}
