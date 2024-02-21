fn main() {
    let numbers = vec![5, 4, 3, 2, 1, 6, 7, 8, 9, 10];
    println!("mean = {}", mean(&numbers));

    let numbers = vec![50, 1, 2, 80, 100];
    println!("mode= {}", mode(&numbers));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum / (numbers.len() as i32)
}

fn mode(numbers: &Vec<i32>) -> i32 {
    return if numbers.is_empty() {
        0
    } else {
        let mut numbers = numbers.clone();
        numbers.sort();
        let middle = numbers.len() / 2;
        numbers[middle]
    };
}
