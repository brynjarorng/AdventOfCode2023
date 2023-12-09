use core::num;
use std::fs;

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

fn star_1<'a>(split_lines: Vec<Vec<&'a str>>) {
    let mut accumulator = 0;
    let mut d1 = 'a';
    let mut d2 = 'a';
    let mut conc: String;

    for line in split_lines.iter() {
        conc = "".to_string();

        // Find first value
        for c in line[0].chars() {
            if c.is_numeric() {
                d1 = c;
                break;
            }
        }
        
        // Find last value
        for c in line[0].chars().rev() {
            if c.is_numeric() {
                d2 = c;
                break;
            }
        }

        conc.push_str(&d1.to_string());
        conc.push_str(&d2.to_string());
        
        accumulator += conc.parse::<i32>().unwrap()
    }

    println!("{}", accumulator);
}

fn get_c_d<'a>(fragment: &'a str, reverse: bool) -> String {
    let num_list = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for i in  0..num_list.len() {
        let mut num = num_list.get(i).unwrap().to_string();

        if reverse {
            num = num.chars().rev().collect::<String>();
        }
        
        if num.len() < fragment.len() {
            match fragment.get(0..num.len()).unwrap() == num {
                true => return (i).to_string(),
                _ => continue
            }
        } else {
            break;
        }
    }

    return "".to_string();
}

fn star_2<'a>(split_lines: Vec<Vec<&'a str>>) {
    let mut accumulator = 0;
    let mut d1 = 'a';
    let mut d2 = 'a';
    let mut conc: String;

    for line in split_lines.iter() {
        d1 = 'a';
        d2 = 'a';
        conc = "".to_string();
        
        // Change words for ints
        let line_altered = line[0].to_string();

        // Find first value
        for c in 0..line_altered.len() {
            let word = line_altered.chars().nth(c).unwrap();

            if word.is_numeric() {
                d1 = word;
                break;
            } else {
                let tmp = get_c_d(line_altered.get(c..).unwrap(), false);

                if tmp.len() > 0 {
                    d1 = tmp.chars().nth(0).unwrap();
                    break;
                }
            }
        }
        
        // Find last value
        let line_rev = line_altered.chars().rev().collect::<String>();
        for c in 0..line_rev.len() {
            let word = line_rev.chars().nth(c).unwrap();

            if word.is_numeric() {
                d2 = word;
                break;
            } else {
                let tmp = get_c_d(line_rev.get(c..).unwrap(), true);

                if tmp.len() > 0 {
                    d2 = tmp.chars().nth(0).unwrap();
                    break;
                }
            }
        }

        conc.push_str(&d1.to_string());
        conc.push_str(&d2.to_string());
        
        println!("{}", line_altered);
        println!("{}", conc);
        println!("{}", "");

        accumulator += conc.parse::<i32>().unwrap()
    }

    println!("{}", accumulator);
}



fn main() {
    let file_contents = fs::read_to_string("./src/input").expect("File should have been read");
    let split_lines = words_by_line(&file_contents);

    // star_1(split_lines.clone());
    star_2(split_lines.clone());
}

