use std::fs;

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

// fn chall1<'a>(input_lines: &'a Vec<&'a str>) -> Vec<GameInfo> {
fn chall1<'a>(input_lines: &'a Vec<&'a str>) -> i32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut acc = 0;
    let mut success;

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
            let val = ball_parts[0].parse::<i32>().unwrap();
            if ball_parts[1] == "red" {
                if val > red_max {
                    success = false
                }
            }
            if ball_parts[1] == "blue" {
                if val > blue_max {
                    success = false
                }
            }
            if ball_parts[1] == "green" {
                if val > green_max {
                    success = false
                }
            }   
        }

        if success {
            acc += game.id;
        }
    }

    return acc
}

fn chall2<'a>(input_lines: &'a Vec<&'a str>) -> i32 {
    let mut acc = 0;

    for games in input_lines {
        let tmp = games;
        
        let split_string = tmp.split(':').collect::<Vec<&str>>();
    
        let mut game: GameInfo = GameInfo::default();
    
        let game_id = split_string[0].get(5..).unwrap().parse::<i32>().unwrap();
        game.id = game_id;
    
        let balls = split_string[1].replace(";", ",");
    
        for ball in balls.split(",") {
            let ball_parts = ball.split_whitespace().collect::<Vec<&str>>();
            let val = ball_parts[0].parse::<i32>().unwrap();
            if ball_parts[1] == "red" {
                if val > game.red {
                    game.red = val;
                }
            }
            if ball_parts[1] == "blue" {
                if val > game.blue {
                    game.blue = val;
                }
            }
            if ball_parts[1] == "green" {
                if val > game.green {
                    game.green = val;
                }
            }   
        }

        acc += game.red * game.blue * game.green;
    }

    return acc
}

fn main() {
    let file_contents: String = fs::read_to_string("./src/input").expect("File should have been read");
    let split_lines = words_by_line(&file_contents);

    let chall1 = chall1(&split_lines);
    let chall2 = chall2(&split_lines);

    println!("{:?}", chall1);
    println!("{:?}", chall2);
}
