use std::io;
use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let part1 = part_one();
    print!("Part 1: {}\n", part1);

    let part2 = part_two();
    print!("Part 2: {}\n", part2);
    
    
}

fn part_one() -> i32 {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(&file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() == 2 {          
            let opponent_choice = tokens[0].chars().next().unwrap();
            let your_choice = tokens[1].chars().next().unwrap();            
            
            // Calculate the score for this round
            let round_score = match (opponent_choice, your_choice) {
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // Win
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // Tie
                _ => 0, // Lose
            };

            // Update the total score
            total_score += round_score + match your_choice {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            };
        }
    }

    total_score

}

fn part_two() -> i32 {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(&file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let tokens: Vec<_> = line.split_whitespace().collect();
        if tokens.len() == 2 {          
            let opponent_choice = tokens[0].chars().next().unwrap();
            let result = tokens[1].chars().next().unwrap();            
            let round_score;
            // Calculate the score for this round
            round_score = match result {
                'X' => match opponent_choice {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => 0,
                },
                'Y' => match opponent_choice {
                    'A' => 4,
                    'B' => 5,
                    'C' => 6,
                    _ => 0,
                },
                'Z' => match opponent_choice {
                    'A' => 8,
                    'B' => 9,
                    'C' => 7,
                    _ => 0,
                },
                _ => 0,
            };

            // Update the total score
            total_score += round_score;
        }
    }

    total_score
}
