use std::{
    fs::File,
    io::prelude::*,
    string::*,
};

fn main(){
    let mut file = File::open("day3.txt").expect("file not found");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("something went wrong reading the file");
    part1(&buf);

}


fn part1(buf: &str){
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    let mut count;
    for line in buf.lines(){
        count = 0;
        for c in line.chars(){
            if c == '1'{
                count += 1;
            }
        }        
        if count > line.chars().count()/2{
            gamma.push('1');
            epsilon.push('0');
        }
        else{
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("part 1 ans: {},{}", gamma, epsilon);

    let intval = u256::from_str_radix("000100110011100101", 2).unwrap();
    println!("{}", intval);


    // let gamma_dec = usize::from_str_radix(gamma.as_str(), 2).unwrap();
    // let epsilon_dec = usize::from_str_radix(epsilon.as_str(), 2).unwrap();
    // getting an overflow on these expressions
   
    // println!("part 1 ans: {}", gamma_dec * epsilon_dec);
}