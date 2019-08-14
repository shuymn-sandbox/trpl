fn main() {
    let a1 = [1, 2, 3];
    println!("a has {} elements", a1.len());

    let names = ["Graydon", "Brian", "Niko"];
    println!("This second name is: {}", names[1]);

    let _a2 = [0, 1, 2, 3, 4];
    let _complete = &_a2[..]; // [0, 1, 2, 3, 4]
    let _middle = &_a2[1..4]; // [1, 2, 3]

    let (x1, _y1, _z1) = (1, 2, 3);
    println!("x1 is {}", x1);

    let tuple = (1, 2, 3);
    let x2 = tuple.0;
    let _y2 = tuple.1;
    let _z2 = tuple.2;
    println!("x2 is {}", x2);
}
