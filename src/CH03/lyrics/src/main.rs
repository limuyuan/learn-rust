fn main() {
    let num_order = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimminng",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drumers drummingn",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            num_order[i]
        );
        match i {
            0 => println!("A partridge in a pear tree."),
            _ => {
                for j in (0..i).rev() {
                    println!("{},", presents[j]);
                }
                println!("And a partridge in a pear tree.");
            }
        }
        println!();
    }
}
