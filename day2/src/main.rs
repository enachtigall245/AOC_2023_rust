use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Sample data for known counts of each color
    let mut marble_counts = HashMap::new();
    marble_counts.insert("red", 12);
    marble_counts.insert("green", 13);
    marble_counts.insert("blue", 14);

    let mut sum_ids = 0;
    let mut power_games = 0;

    // Open the file
    if let Ok(file) = File::open("day2input.txt") {
        let reader = BufReader::new(file);

        // Process each line as a game entry
        for line in reader.lines() {
            if let Ok(game) = line {
                let game_info: Vec<&str> = game.split(':').collect();
                if game_info.len() != 2 {
                    println!("Invalid game format: {}", game);
                    continue;
                }

                let game_id = game_info[0];
                let game_data = game_info[1].trim();

                if is_valid_game(game_data, &marble_counts) {
                    //println!("Game {} is valid", game_id);
                    if let Ok(number) = game_id.trim_start_matches("Game ").parse::<u32>() {
                        sum_ids += number;
                    } 
                } else {
                    //println!("Game {} is not valid", game_id);
                }

                let game_counts = max_color_counts(game_data);
                let mut running_prod = 1;
                for (_key, value) in &game_counts {
                    running_prod *= value;
                }
                power_games += running_prod;

            }
        }
    } else {
        println!("Error reading the file");
    }

    println!("Sum of Valid Games: {}",sum_ids);
    println!("Minimum Colors Power Sum of All Games {}",power_games);
}

fn is_valid_game(game: &str, counts: &HashMap<&str, u32>) -> bool {
    let pulls: Vec<&str> = game.split(';').collect();
    
    for pull in pulls {
        let marbles: Vec<&str> = pull.split(',').collect();
        
        for marble in marbles {
            let parts: Vec<&str> = marble.trim().split_whitespace().collect();
            if parts.len() != 2 {
                return false;
            }
            
            let count = parts[0].parse::<u32>().unwrap_or(0);
            let color = parts[1];
            
            if let Some(&value) = counts.get(color) {
                if count > value {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    
    true
}

fn max_color_counts(game: &str) -> HashMap<&str, u32> {
    let mut max_counts = HashMap::new();
    let pulls: Vec<&str> = game.split(';').collect();
    for pull in pulls {
        let marbles: Vec<&str> = pull.split(',').collect();
        for marble in marbles {
            let parts: Vec<&str> = marble.trim().split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }
            let count = parts[0].parse::<u32>().unwrap_or(0);
            let color = parts[1];
            if let Some(&max) = max_counts.get(color) {
                if count > max {
                    max_counts.insert(color, count);
                }
            } else {
                max_counts.insert(color, count);
            }
        }
    }
    max_counts
}