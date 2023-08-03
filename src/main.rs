fn main() {
    println!("{}\n", format_f64_eng(std::f64::consts::PI, None));
    println!("{}\n", format_f64_eng(333.3, None));
    println!("{}\n", format_f64_eng(666.6, None));
    println!("{}\n", format_f64_eng(22.2e3, None));
    println!("{}\n", format_f64_eng(77.7e3, None));
}

/// Returns f64 as string in engineering notation with last digit
/// rounded to nearest rather than truncated.
/// # Arguments
/// * x - value to be formatted
/// * s - number of significant figures, defaults to 3
fn format_f64_eng(x: f64, s: Option<usize>) -> String {
    let s = s.unwrap_or(3);

    println!("x: {x}");

    let exp: usize = x.abs().log10().floor() as usize - x.abs().log10().floor() as usize % 3;
    println!("exp: {exp}");

    let n_left_of_dec: usize = x.abs().log10().floor() as usize % 3 + 1;
    println!("n_left_of_dec: {}", n_left_of_dec);

    match x {
        x if exp < 3 => {
            format!("{x:.*}", s - n_left_of_dec)
        }
        x => {
            let x = x / 10_f64.powf(exp as f64);
            format!("{x:.*}e{exp}", s - n_left_of_dec)
        }
    }
}
