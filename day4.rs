fn main(){
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let input = input.parse::<i32>().unwrap();
        println!("{}", fib(input));
    }
}

fn fib(n: i32) -> i32 {
    // memoize
    let mut memo = vec![0; n as usize + 1];
    fib_helper(&mut memo, n)
}

fn fib_helper(memo: &mut Vec<i32>, n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if memo[n as usize] != 0 {
        return memo[n as usize];
    }
    memo[n as usize] = fib_helper(memo, n-1) + fib_helper(memo, n-2);
    memo[n as usize]
}