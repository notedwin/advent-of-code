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
    // references &var are immutable
    // ownership means I can use the variable withing the scope but scope can change to a sub scope

    let x = "hello";
    func(x);

    fn func(x: &str) {
        println!("{}", x);
    }

    // can't use x after this scope
    // however if we have copy x, we can still use it

    let x = 12;
    fuc(x);

    fn fuc(x: i32) {
        println!("{}", x);
    }

    println!("{}", x);

    // can get around this by using a mutable variable
    // however we cant use more than 1 mut variable in a scope, to avoid data races

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // error handling

    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    //v.get(10).unwrap();

    match v.get(10) {
        Some(x) => println!("{}", x),
        None => println!("none"),
    }

    let x = "5";

    match x.parse::<i32>() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}