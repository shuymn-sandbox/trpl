fn main() {
    println!("Hello, world!");

    another_function_1();
    another_function_2(2);
    another_function_3(10, 30);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();

    println!("The value of z is: {}", z);

    let z = plus_one(z);

    println!("The value of z is: {}", z);
}

fn another_function_1() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_3(x:i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}