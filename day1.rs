use std::fs::File;
use std::io::prelude::*;

fn main() {
    part1();
    part2();
}

fn part1(){
    let mut f = File::open("day1.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    // convert to ints
    let mut arr: Vec<i32> = Vec::new();
    // create int to hold count
    let mut greater = 0;
    for line in lines {
        let line = line.trim();
        if line.len() > 0 {
            let num: i32 = line.parse().unwrap();
            //if num is greater than previosu num, set prev to num
            
            // check if arr is empty
            if arr.len() == 0{
                // continue
                // ???
            }
            // else if
            else if num > arr[arr.len() - 1] {
                greater += 1;
            }

            arr.push(num);
        }
    }
    println!("part 1 ans: {}", greater);
}

fn part2(){
    let mut f = File::open("day1.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    // convert to ints
    let mut arr: Vec<i32> = Vec::new();
    // create int to hold count
    let mut greater = 0;
    for line in lines {
        if line.len() > 0 {
            let num: i32 = line.parse().unwrap();
            arr.push(num);
        }
    }
    // create an array of slices of size 3
    let mut arr_slices: Vec<&[i32]> = Vec::new();

    // create a slice of size 3
    let mut slice: Vec<i32> = Vec::new();
    // iterate through the array
    for num in arr {
        // if slice is empty, add num to slice
        if slice.len() == 0 {
            slice.push(num);
        }
        // else if slice is full, add slice to arr_slices
        else if slice.len() == 3 {
            arr_slices.push(slice.as_slice());
            slice.clear();
            slice.push(num);
        }
        // else add num to slice
        else {
            slice.push(num);
        }
    }

    // get count of 

    println!("part 2 ans: {}", arr.len());
}