// fn fibbo_nth(n: i64) -> i64 {
//     if n == 0 || n == 1 {
//         return n;
//     } else {
//         fibbo_nth(n - 2) + fibbo_nth(n - 1)
//     }
// }

fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut last_two: (usize, usize) = (0, 1);
            for _ in 2..n {
                last_two = (last_two.1, last_two.0 + last_two.1);
            }
            last_two.1
        }
    }
}

// pub fn show_fibbo_sequence(n: i64) {
//     let base_fibbo_sequence = [0, 1, 1];
//     let mut last_sum = 2;
//     for i in 3..n {
//         println!("{}", base_fibbo_sequence);
//     }
// }
pub fn calculate_fibo_nth(n: usize) {
    println!("{}", fib(n));
}
