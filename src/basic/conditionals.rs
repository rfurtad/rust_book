pub fn read_age(age: i32) {
    if age < 12 {
        println!("You are a child");
    } else if age < 18 {
        println!("You are a teen");
    } else if age < 30 {
        println!("You are a young adult");
    } else if age < 50 {
        println!("You are an adult");
    } else {
        println!("You are senior")
    }
}
