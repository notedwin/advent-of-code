fn main() {
    let x = 5;

    let x = x + 1;

    // shadowing and scopes9p
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // does not compile: can't mutate type
    //let mut spaces = "   ";
    //spaces = spaces.len();
    

    // scalar and compound
    // _ is a visual sperator, 98_222, 0xff, 0o77, 0b111
    // scalar is typical types, compounds are tuples and arrs
    

}
