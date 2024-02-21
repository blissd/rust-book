fn main() {
    let fs = [-1.0, 0.0, 32.0, 100.0];

    for f in fs {
        let c = fahrenheit_to_celsius(f);
        println!("{f} Fahrenheit is {c} Celsius");
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    ((f - 32.0) * 5.0) / 9.0
}
