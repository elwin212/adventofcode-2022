use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashSet;
use itertools::Itertools;


fn main() {
    let p1 = part_one();
    println!("Part 1: {}", p1);

    let p2 = part_two();
    println!("Part 2: {}", p2);
}


fn part_one () -> i32 {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut res: i32 = 0;
    for line in reader.lines() {        
        if let Ok(line) = line {            
            let first_half = &line[0..line.len()/2];
            let second_half = &line[line.len()/2..line.len()];
            let set1: HashSet<_> = first_half.chars().collect();
            let set2: HashSet<_> = second_half.chars().collect();
            let dup = set1.intersection(&set2);
                        
            dup.for_each(|c| {
                let num = map_char_to_number(*c).unwrap();
                res += num as i32;
            });            
        }
    }
    res
}

fn map_char_to_number(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None, // Character is not in the specified range
    }
}

fn part_two () -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let res = input
                    .lines()
                    .map(|line| line.as_bytes())
                    .tuples()
                    .filter_map(|(sack1, sack2, sack3)| {
                        sack1
                            .iter()
                            .find(|item| sack2.contains(item) && sack3.contains(item))
                            .map(|item| match item {
                                b'a'..=b'z' => (item - b'a') + 1,
                                _ => (item - b'A') + 1 + 26,
                            } as i32)
                    })
                    .sum::<i32>();
    res    
}