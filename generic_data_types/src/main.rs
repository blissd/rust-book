fn largest(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);
}
