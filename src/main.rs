fn main() {
    const MAX_POINTS: i64 = 100_000;
    let hehe: i64 = MAX_POINTS.pow(2);

    // variable by default is immutable

    // mutable variable
    let mut hello = "Hello, ".to_string();

    hello.push_str("world!");

    println!("{}", hello);

    hello = "goodbye".to_string();

    println!("bye: {}", hello);

    // create a variable called `x` with the value `1` and a type of `i32`
    let x: i32 = 4;
    // create a variable called `y` with the value `2` and a type of `i32`
    let y: i32 = 2;

    // use the `+` operator to add `x` and `y` together and assign the result to `sum`
    let sum = x + y;
    // use the `-` operator to subtract `y` from `x` and assign the result to `difference`
    let difference = x - y;
    // use the `*` operator to multiply `x` and `y` together and assign the result to `product`
    let product = x * y;
    // use the `/` operator to divide `x` by `y` and assign the result to `quotient`
    let quotient = x / y;
    // use the `%` operator to modulo `x` by `y` and assign the result to `remainder`
    let remainder = x % y;

    // print the value of `sum`
    println!("{}", sum);
    // print the value of `difference`
    println!("{}", difference);
    // print the value of `product`
    println!("{}", product);
    // print the value of `quotient`
    println!("{}", quotient);
    // print the value of `remainder`
    println!("{}", remainder);

    // make my own scope
    {
        let x = x - 2;
        println!("The value of x is: {}", x);
    }

    let x = x + 2;

    println!("The value of x is: {}", x);

    // print constant
    println!("maxpoint x 2: {}", hehe);

    // create a simple forloop
    for x in 0..10 {
        println!("{}", x);
    }

    // create a forloop with a condition
    for x in 0..10 {
        if x % 2 == 0 {
            println!("0 only loop: {}", x);
        } else {
            println!("!0 only loop: {}", x);
        }
    }
}
