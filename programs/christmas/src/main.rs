fn main() {
    let christmas_list = vec![
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree"
    ];

    let mut day = 1;

    while day <= 12 {
        println!("On the {} day of christmas,", day);
        println!("my true love gave to me");
        for i in (1..day+1).rev() {
            println!("{},", christmas_list[12 - i]);
        }
        println!("-----------------------");
        day += 1;
    }
}
