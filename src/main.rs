fn main() {
    println!("{}\n", format_f64_eng(std::f64::consts::PI, None));
    println!("{}\n", format_f64_eng(33.333, Some(7)));
    println!("{}\n", format_f64_eng(66.666, None));
    println!("{}\n", format_f64_eng(333.33, None));
    println!("{}\n", format_f64_eng(666.66, None));
    println!("{}\n", format_f64_eng(3333.3, None));
    println!("{}\n", format_f64_eng(6666.6, None));
    println!("{}\n", format_f64_eng(33.333e6, None));
    println!("{}\n", format_f64_eng(66.666e6, None));
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

    // number of digits left of decimal
    let n_left_of_dec: usize = x.abs().log10().floor() as usize % 3 + 1;
    println!("n_left_of_dec: {}", n_left_of_dec);

    // number of digits right of decimal
    let n_dec = s - n_left_of_dec;

    let x_rounded = (x * n_dec as f64).round() / n_dec as f64;

    match x_rounded {
        x if exp < 3 => {
            format!("{x:.*}", n_dec)
        }
        x => {
            let x = x / 10_f64.powf(exp as f64);
            format!("{x:.*}e{exp}", n_dec)
        }
    }
}
