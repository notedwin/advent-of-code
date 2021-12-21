use std::{
    fs::File,
    io::{prelude::*,
        BufReader},
};


fn main() {
    let file = File::open("day1.txt").expect("file not found");
    let reader = BufReader::new(file);

    // more idomatic way to do this
    // for lines in reader, parse str into 32 bit int then check for errors and collect all the parsed ints

    let arr = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    part1(&arr);
    part2(&arr);
}

fn part1(arr: &Vec<i32>) {
    // get count of number of numbers greater than previous
    // let mut count = 0;

    // arr.iter().enumerate().skip(1).for_each(|(idx, num)| {
    //     if num > &arr[idx - 1] {
    //         count += 1;
    //     }
    // });

    let count = arr.iter().zip(arr.iter().skip(1)).filter(|(a, b)| a < b).count();
    println!("part 1 ans: {}", count);
}

fn part2(arr: &Vec<i32>) {
    // create a shadowing copy of array with sliding window of size 2

    let arr_copy = arr.windows(3).map(|w| w.iter().sum::<i32>()).collect::<Vec<i32>>();

    // for each window of size 3, check if sum is greater than previous window
    let count = arr_copy.iter().zip(arr_copy.iter().skip(1)).filter(|(a, b)| a < b).count();
    
    
    println!("part 2 ans: {}", count);
}