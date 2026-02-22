#[allow(dead_code)]
fn input() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Digite algo");
    println!("Agora é hora de printar o que eu escrevi: {input}")
}
