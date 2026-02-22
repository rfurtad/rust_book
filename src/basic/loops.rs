//song https://www.letras.mus.br/natal/1050132/
pub fn print_christmas_lyrics() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];

    let phrase = "My true love sent to me: ";

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-living",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    // using iterable...bonito, mas é um tópico que ainda não aprendi direito pelo book
    // for (index_day, value_day) in days.iter().enumerate() {
    //     println!("On the {value_day} day of Christmas");
    //     println!("{phrase}");
    //     for index in (0..index_day).rev() {
    //         println!("{}", gifts[index]);
    //     }
    //     println!("\n");
    // }

    // versão pelo que aprendi no book
    for number_of_day in 0..11 {
        println!("On the {} day of Christmas", days[number_of_day]);
        println!("{phrase}");
        for index in (0..number_of_day).rev() {
            println!("{}", gifts[index]);
        }
        println!("\n");
    }
}
