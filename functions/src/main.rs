fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1;
}
