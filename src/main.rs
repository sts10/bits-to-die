fn main() {
    let number_of_words: f64 = 7776 as f64;

    println!("log base 2 of 5 is {}", log_base_2(5 as f64));
    println!("log base 2 of 6 is {}", log_base_2(6 as f64));
    println!("H of 7776 is {}", log_base_2(number_of_words));
    println!("H of 7776^3 is {}", log_base_2(7776_i64.pow(3) as f64));
    println!(
        "H of compromised 3 is {}",
        log_base_2((7776_i64.pow(3) * 5) as f64)
    );
    println!("Hello, world!");
}

fn log_base_2(word_count: f64) -> f64 {
    let base_2: f64 = 2.0;
    return (word_count.ln() / base_2.ln()) as f64;
}
