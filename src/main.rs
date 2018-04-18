use std::io;
fn main() {
    // println!("Enter number of bits to be converted to die");
    // let bits = get_float_from_input() as f64;

    // println!(
    //     "{} bits is equivalent to {} die",
    //     bits,
    //     bits_to_die(bits as f64)
    // );

    println!("Enter number of die to be converted to bits");
    let die = get_float_from_input() as f64;

    println!(
        "{} die is equivalent to {} bits",
        die,
        die_to_bits(die as f64)
    );
}

fn log_base_6(n: f64) -> f64 {
    let six_as_float: f64 = 6.0;
    return (n.ln() / six_as_float.ln()) as f64;
}

fn log_base_2(n: f64) -> f64 {
    let two_as_float: f64 = 2.0;
    return (n.ln() / two_as_float.ln()) as f64;
}

fn bits_to_die(bits: f64) -> f64 {
    // honestly not sure why the following line doesn't cause
    // an overflow when bits is greater than 64

    let possibilities: f64 = 2_f64.powf(bits) as f64;
    return log_base_6(possibilities);
}

fn die_to_bits(die: f64) -> f64 {
    let possibilities: f64 = 6_f64.powf(die) as f64;
    return log_base_2(possibilities);
}

fn get_float_from_input() -> f64 {
    let reader: io::Stdin = io::stdin();
    let mut input_text: String = String::new();
    let result: Result<usize, io::Error> = reader.read_line(&mut input_text);
    if result.is_err() {
        println!("failed to read from stdin");
    }
    let trimmed: &str = input_text.trim();
    let option: Option<f64> = trimmed.parse::<f64>().ok();
    match option {
        Some(i) => return i,
        None => return 0.0,
    };
}
