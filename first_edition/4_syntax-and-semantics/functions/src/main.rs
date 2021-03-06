fn main() {
    print_number(5);
    print_sum(5, 6);
    print_number(add_one(10));

    let f: fn(i32) -> i32 = add_one;
    let six = f(5);
    print_number(six);
    diverges();
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
