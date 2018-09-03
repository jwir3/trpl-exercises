fn main() {
    println!("The Twelve Days of Christmas");
    println!("----------------------------");
    println!("");

    let mut day = 1;

    while day <= 12 {
        println!(
            "On the {} day of Christmas, my true love gave to me {}",
            convert_number_to_word(day),
            build_gift_list_for_day(day)
        );
        day = day + 1;
    }
}

fn convert_number_to_word(num: usize) -> String {
    let word_conversions = [
        "", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];

    word_conversions[num].to_string()
}

fn build_gift_list_for_day(day: usize) -> String {
    if day == 1 {
        gift_for_day(day) + "."
    } else {
        let mut gift_list = String::new();
        let mut days_left = day;
        while days_left > 1 {
            let next_gift = gift_for_day(days_left);
            if day > 2 && days_left != day {
                gift_list = gift_list + ", " + &next_gift;
            } else {
                gift_list = gift_list + &next_gift;
            }

            days_left = days_left - 1;
        }

        let first_gift = gift_for_day(1);
        if day == 2 {
            gift_list = gift_list + " and " + &first_gift + ".";
        } else {
            gift_list = gift_list + ", and " + &first_gift + ".";
        }
        gift_list
    }
}

fn gift_for_day(day: usize) -> String {
    let gifts = [
        "",
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    gifts[day].to_string()
}
