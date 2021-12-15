use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("test.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut arr: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut board_num = 0;

    // create empty nums: str arr
    let num: <Vec<&str>> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        // if first line read in, set the number of lines
        if idx == 0 {
            let nums = line.split(",").collect();
            println!("{}", nums.len());
        }
        // strip line of whitespace
        let line = line.trim();
        // else if line is not empty, process
        // else if
        



        elif line.len() > 0{
            // append an array to an 
        }
        else {
            // increment board number
            arr[board_num] = board;
            board_num += 1;
        }
    }
}