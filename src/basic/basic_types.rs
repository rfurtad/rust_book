pub fn types() {
    // Explicit types
    let explicit_integers: i32 = 1;
    let explicit_float: f64 = 3.22;
    let explicit_char: char = 'a';
    let explicit_boolean: bool = true;
    let explicit_string: String = String::from("areu");
    let explicit_sliced_string: &str = "string";
    let infered_integer = 1;
    let infered_string = '1';
    let infered_boolean = false;
    let infered_float = 3.22;

    println!("Explicit integers: {}", explicit_integers);
    println!("Explicit float: {}", explicit_float);
    println!("Explicit char: {}", explicit_char);
    println!("Infered string: {}", infered_string);
    println!("Infered boolean: {}", infered_boolean);
    println!("Infered integer: {}", infered_integer);
    println!("Infered float: {}", infered_float);
    println!("Explicit string: {}", explicit_string);
    println!("Explicit boolean: {}", explicit_boolean);
    println!("Explicit sliced string: {explicit_sliced_string}");
}
