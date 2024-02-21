fn main() {
    let days_and_gifts = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three french hens"),
        ("fourth", "four colly birds"),
        ("fifth", "five gold rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eigth", "eight maids a-milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelth", "twelve drummers drumming"),
    ];

    let mut index = 0;

    while index < 12 {
        let (day, gift) = days_and_gifts[index];
        println!("On the {day} of Christmas my true love sent to me");
        println!("{gift}");

        for count_down in (0..index).rev() {
            let (_, gift) = days_and_gifts[count_down];
            if count_down == 0 {
                println!("and {gift}");
            } else {
                println!("{gift}")
            }
        }
        println!("");        
        index += 1;
    }
}

