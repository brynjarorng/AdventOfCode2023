use std::{fs, any};

struct GameInfo {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn words_by_line<'a>(s: &'a str) -> Vec<&'a str> {
    s.lines().map(|line| {
        line
    }).collect()
}

// fn parse_data<'a>(input_lines: &'a Vec<&'a str>) -> Vec<GameInfo> {
fn parse_data<'a>(input_lines: &'a Vec<&'a str>) -> i32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut acc = 0;
    let mut success = true;

    let mut parsed_games: Vec<GameInfo> = vec![];

    for games in input_lines {
        success = true;
        let tmp = games;
        
        let split_string = tmp.split(':').collect::<Vec<&str>>();
    
        let mut game: GameInfo = GameInfo::default();
    
        let game_id = split_string[0].get(5..).unwrap().parse::<i32>().unwrap();
        game.id = game_id;
    
        let balls = split_string[1].replace(";", ",");
    
        for ball in balls.split(",") {
            let ball_parts = ball.split_whitespace().collect::<Vec<&str>>();
            if ball_parts[1] == "red" {
                game.red += ball_parts[0].parse::<i32>().unwrap();
                if ball_parts[0].parse::<i32>().unwrap() > red_max {
                    success = false
                }
            }
            if ball_parts[1] == "blue" {
                game.blue += ball_parts[0].parse::<i32>().unwrap();
                if ball_parts[0].parse::<i32>().unwrap() > blue_max {
                    success = false
                }
            }
            if ball_parts[1] == "green" {
                game.green += ball_parts[0].parse::<i32>().unwrap();
                if ball_parts[0].parse::<i32>().unwrap() > green_max {
                    success = false
                }
            }   
        }

        if success {
            acc += game.id;
        }

        parsed_games.push(game);
    }

    return acc
}

fn main() {
    let file_contents: String = fs::read_to_string("./src/input").expect("File should have been read");
    let split_lines = words_by_line(&file_contents);

    let parsed_data = parse_data(&split_lines);

    println!("{:?}", parsed_data);
}
