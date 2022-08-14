use std::io;

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

    // playin around with data types
    let x: u32 = 5;
    let y: i32 = -5;

    println!("u32 cant assigned int really");
    println!("x: {}", x);
    println!("y: {}", y);

    let floaten: f32 = 10.32;
    println!("{}", floaten);

    let mut bul: bool = true;
    println!("{}", bul);

    // tuple -> kinda like object
    let text: String = "hello".to_string();

    let mut tup: (i32, f64, u8, String) = (500, 6.4, 1, text);
    // change the value of the tuple
    tup.3 = "this is a string".to_string();
    println!("{}", tup.3);

    let arr: [i32; 4] = [1, 2, 3, 4];
    // loop arr and print each value
    for x in arr.iter() {
        println!("arr in LOOP {}", x);
    }

    // array of tuple
    let mut arr2: [(&str, i32); 4] = [("hello", 1), ("world", 2), ("!", 3), ("!", 4)];
    println!("{}", arr2[0].0);
    println!("{}", arr2[0].1);

    // mutate arr2
    arr2[0].1 = 5;
    println!("{}", arr2[0].1);
    arr2[0].0 = "hello world";
    println!("{}", arr2[0].0);

    // create a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // create a new instance of the struct
    let mut user1 = User {
        username: "USE".to_string(),
        email: "EM".to_string(),
        sign_in_count: 1,
        active: true,
    };

    // change the value of the struct
    user1.username = "hello".to_string();
    user1.email = "world".to_string();
    user1.sign_in_count = 2;
    user1.active = false;

    // print the value of the struct
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("active: {}", user1.active);

    // console input...
    let mut input = String::new();
    // loop until input is not empty
    while input.trim().len() == 0 {
        println!("Please enter your name: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    println!("Hello {}", input.trim());
}
