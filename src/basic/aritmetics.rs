pub fn convert_farenheit_to_celsions() {
    let mut input = String::new();
    println!("Digite o valor da temperatura: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Digite o valor correto");
    let temp: f64 = input.trim().parse().unwrap();
    convert(temp);
}
pub fn convert(tp_farenheit: f64) {
    let tp_celsius = 5.0 * (tp_farenheit - 32.0) / 9.0;
    println!("{tp_celsius}");
}
