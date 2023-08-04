//! Module containing function (and maybe trait?) to format f64 in [engineering
//! notation](https://en.wikipedia.org/wiki/Engineering_notation)

/// Returns f64 as string in [engineering
/// notation](https://en.wikipedia.org/wiki/Engineering_notation) with last digit rounded to nearest
/// rather than truncated.
/// # Arguments
/// * x - value to be formatted
/// * s - number of significant figures, defaults to 3
fn format_f64_eng(x: f64, s: Option<usize>) -> Result<String, String> {
    let s = s.unwrap_or(3);
    if s < 3 {
        return Err("Number of significant figures `s` cannot be less than 3!".to_string());
    }

    println!("x: {x}");

    // engineering notation exponent
    let exp_eng: usize = x.abs().log10().floor() as usize - x.abs().log10().floor() as usize % 3;

    let exp_sci = x.abs().log10().floor() as usize;

    // number of digits left of decimal _after_ formatting for engineering notation, should never
    // exceed 3
    let n_left_of_dec: usize = x.abs().log10().floor() as usize % 3 + 1;

    println!("{}", exp_sci);

    assert!(
        n_left_of_dec <= 3,
        "n_left_of_dec: {} exceeds 3",
        n_left_of_dec
    );

    let n_dec = s - n_left_of_dec;

    let mut x_base = match exp_eng {
        _ if exp_eng <= 2 => x,
        _ => x / 10_f64.powi(exp_eng as i32),
    };

    x_base = match n_left_of_dec {
        _ if n_left_of_dec == 3 => x_base.round(),
        _ => {
            (x_base * 10_f64.powi((s - n_left_of_dec) as i32)).round()
                * 10_f64.powf(-((s - n_left_of_dec) as f64))
        }
    };

    match exp_eng {
        _ if exp_eng <= 3 => Ok(format!("{x_base:.*}", n_dec)),
        _ => Ok(format!("{x_base:.*}e{}", n_dec, exp_eng)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pi() {
        assert_eq!(
            format_f64_eng(std::f64::consts::PI, None),
            Ok(String::from("3.14"))
        );
    }
    #[test]
    fn test_33p333() {
        assert_eq!(
            format_f64_eng(33.333, Some(7)),
            Ok(String::from("33.33300"))
        );
    }
    #[test]
    fn test_66p666() {
        assert_eq!(format_f64_eng(66.666, None), Ok(String::from("66.7")));
    }
    #[test]
    fn test_333p33() {
        assert_eq!(format_f64_eng(333.33, None), Ok(String::from("334")));
    }
    #[test]
    fn test_666p66() {
        assert_eq!(format_f64_eng(666.66, None), Ok(String::from("667")));
    }
    #[test]
    fn test_3333p3() {
        assert_eq!(format_f64_eng(3333.3, None), Ok(String::from("334e3")));
    }
    #[test]
    fn test_6666p6() {
        assert_eq!(format_f64_eng(6666.6, None), Ok(String::from("667e3")));
    }
    #[test]
    fn test_33p333e6() {
        assert_eq!(format_f64_eng(33.333e6, None), Ok(String::from("33.4e6")));
    }
    #[test]
    fn test_66p666e6() {
        assert_eq!(format_f64_eng(66.666e6, None), Ok(String::from("66.7e6")));
    }
}
