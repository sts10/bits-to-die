use std::io;
use std::str::FromStr;
fn main() {
    println!("Enter number of bits or die to be converted to die and bits");
    let input: f64 = ensure("Please try again. Enter a number (a float)").unwrap();

    println!("{} bits is equivalent to {} die", input, bits_to_die(input));

    println!("{} die is equivalent to {} bits", input, die_to_bits(input));
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

fn ensure<T: FromStr>(try_again: &str) -> io::Result<T> {
    loop {
        let line = match gets() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        match line.parse() {
            Ok(res) => return Ok(res),
            // otherwise, display inputted "try again" message
            // and continue the loop
            Err(_e) => {
                eprintln!("{}", try_again);
                continue;
            }
        };
    }
}

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches("\n").to_string()),
        Err(error) => Err(error),
    }
}

#[test]
fn can_calc_bits_to_die() {
    assert_eq!(bits_to_die(12.92), 4.998138269470278);
    assert_eq!(bits_to_die(77.55), 30.0004352010387);
}

#[test]
fn can_calc_die_to_bits() {
    assert_eq!(die_to_bits(5 as f64), 12.92481250360578);
}
