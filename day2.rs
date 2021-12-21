use std::{
    fs::File,
    io::prelude::*,
};

fn main() {
    let mut file = File::open("day2.txt").expect("file not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("something went wrong reading the file");
    part1(&buf);
    part2(&buf);
}

fn part1(buf: &str) {
    let (x,y) = buf
        .lines()
        .filter_map(|l| {
            l.split_once(" ")
                .map(|(com,val)| (com,val.parse::<i32>().unwrap()))
        })
        .fold((0,0), |(x,y), (com,val)| {
            match com {
                "forward" => (x + val, y),
                "down" => (x, y + val),
                "up" => (x,y - val),
                _ => unreachable!()
            }
        });
    println!("part 1 ans: {}", x * y);
}

fn part2(buf: &str) {
    let (x,y, _aim) = buf
        .lines()
        .filter_map(|l| {
            l.split_once(" ")
                .map(|(com,val)| (com,val.parse::<i32>().unwrap()))
        })
        .fold((0,0,0), |(x,y,aim), (com,val)| {
            match com {
                "forward" => (x + val, y+aim*val, aim),
                "down" => (x, y, aim + val),
                "up" => (x,y, aim - val),
                _ => unreachable!()
            }
        });
    println!("part 2 ans: {}", x * y);
}
