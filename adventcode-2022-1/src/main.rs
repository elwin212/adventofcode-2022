use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::collections::BinaryHeap;


fn main() {
    // Open the file for reading
    let file = File::open("input.txt").expect("Failed to open file");
    let top_elf = find_max_cal(&file);

    // Reopen the file for the second function
    let file = File::open("input.txt").expect("Failed to open file");
    let top_three_elf = find_top_three(&file);
    
    

    println!("Top one: {}", top_elf);
    println!("Top three: {}", top_three_elf);

   
}

fn find_max_cal(file: &File) -> i32 {
    let reader = BufReader::new(file);    

    let mut _max = i32::MIN;
    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            // Check if the line is empty or contains only whitespace
            if line.trim().is_empty() {
                
                if sum > _max {
                    _max = sum;
                }

                sum = 0;
                
            } else {
                // Parse the line as an integer and push it into the current_numbers vector
                if let Ok(number) = line.parse::<i32>() {                    
                    sum += number;
                }
            }
        }
    }

        
    _max

}

fn find_top_three(file: &File) -> i32 {
    let reader = BufReader::new(file);    
    let mut max_heap = BinaryHeap::new();
    let mut res = 0;
    let mut sum = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            // Check if the line is empty or contains only whitespace
            if line.trim().is_empty() {
                // Process the current set of numbers
                max_heap.push(sum);
                sum = 0;
                // Clear the current_numbers vector for the next set                
            } else {
                // Parse the line as an integer and push it into the current_numbers vector
                if let Ok(number) = line.parse::<i32>() {                    
                    sum += number;
                }
            }
        }
    }

    for _i in 0..3 {
        if let Some(max) = max_heap.pop() {
            res += max;
        } else {
            // Handle the case where there are fewer than three elements in the heap
            break;
        }
    }

    res
}
