use std::collections::HashSet;
use regex::{Captures, Match, Regex};

fn main() {
    let input = include_str!("../input.txt");
    let game_config: [i32; 3] = [12, 13, 14];

    //let result = process_games(input, game_config);
    let result = minimal_configurations(input);
    println!("Result: {}", result);
}

/*
 * Process games from https://adventofcode.com/2023/day/2
 *
 * @param input: &str -> Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
 * @param game_config: [i32; 3] -> [red, green, blue]
 * @return i32 sum of all games which are possible
 */
fn process_games(input: &str, game_config: [i32; 3]) -> i32 {

    let re = Regex::new(r"((?P<red>\d+) red)|((?P<blue>\d+) blue)|((?P<green>\d+) green)").unwrap();
    let rg = Regex::new(r"Game (?P<game_id>\d+):").unwrap();

    let mut sum_all_games: i32 = 0;
    let mut all_valid_games: HashSet<i32> = HashSet::new();
    for (i, line) in input.lines().enumerate() {

        let Some(caps) = rg.captures(line) else { panic!("Error processing line with Game x: -> Check input.") };
        let game_id = caps["game_id"].parse::<i32>().expect("Error parsing game id -> Check input.");
        let games = line.split(";").collect::<Vec<&str>>();

        let mut games_result:Vec<i32> = Vec::new();
        for game in games {
            let mut sum_game: [i32; 3] = [game_config[0], game_config[1], game_config[2]];

            for capture in re.captures_iter(game) {
                if let Some(red_match) = capture.name("red") {
                    sum_game[0] -= red_match.as_str().parse::<i32>().unwrap();
                }            
                if let Some(green_match) = capture.name("green") {
                    sum_game[1] -= green_match.as_str().parse::<i32>().unwrap();
                }
                if let Some(blue_match) = capture.name("blue") {
                    sum_game[2] -= blue_match.as_str().parse::<i32>().unwrap();
                }
            }
            println!("Game {} results {:?}", game_id, sum_game);

            match sum_game.iter().all(|&x| x >= 0i32){
                true => games_result.push(1),
                false => games_result.push(0)
            };
        }
        if games_result.iter().all(|&x| x == 1i32) {
            all_valid_games.insert(game_id);
        }
        sum_all_games = all_valid_games.iter().sum::<i32>();
    }
    return sum_all_games;
}

fn minimal_configurations(input: &str) -> i32 {

    let re = Regex::new(r"((?P<red>\d+) red)|((?P<blue>\d+) blue)|((?P<green>\d+) green)").unwrap();
    let mut sum_all_games: i32 = 0;

    for line in input.lines(){
        let games = line.split(";").collect::<Vec<&str>>();
        let mut min_config: [i32; 3] = [0, 0, 0];

        for game in games {
            for capture in re.captures_iter(game) {
                if let Some(red_match) = capture.name("red") {
                    min_config[0] = parse_and_write(min_config[0], red_match);
                }
                if let Some(green_match) = capture.name("green") {
                    min_config[1] = parse_and_write(min_config[1], green_match);
                }
                if let Some(blue_match) = capture.name("blue") {
                    min_config[2] = parse_and_write(min_config[2], blue_match);
                }
            }
            println!("{} results ==> {:?}", line, min_config);
        }
        sum_all_games += (min_config[0] * min_config[1] * min_config[2]);
    }

    return sum_all_games;
}

fn parse_and_write(value:i32, matching: Match) -> i32{
    let tmp = matching.as_str().parse::<i32>().unwrap();
    return if tmp > value { tmp } else { value };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_game1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_config: [i32; 3] = [12, 13, 14];
        let result = process_games(input, game_config);
        assert_eq!(result, 1); // 1 means true here => is processiable
    }

    #[test]
    fn test_process_game2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game_config: [i32; 3] = [12, 13, 14];
        let result = process_games(input, game_config);
        assert_eq!(result, 2); // 1 means true here => is processiable
    }

    #[test]
    fn test_process_game3() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game_config: [i32; 3] = [12, 13, 14];
        let result = process_games(input, game_config);
        assert_eq!(result, 0); // 0 means true here => is processiable
    }

    #[test]
    fn test_proccess_multiple_games(){
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let game_config: [i32; 3] = [12, 13, 14];
        let result = process_games(input, game_config);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_minimal_configuration_game1(){
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = minimal_configurations(input);

        assert_eq!(result, 48);
    }

    #[test]
    fn test_minimal_configuration_all_games(){
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = minimal_configurations(input);
        assert_eq!(result, 2286);
    }
}