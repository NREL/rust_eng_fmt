//! Module containing trait to format f64 in [engineering
//! notation](https://en.wikipedia.org/wiki/Engineering_notation)

/// Trait providing method for formatting numbers in [engineering
/// notation](https://en.wikipedia.org/wiki/Engineering_notation)
pub trait FormatEng {
    /// Method for converting numeric value into formatted string with engineering notation
    fn format_eng(&self, sf: Option<usize>) -> String;
}

impl FormatEng for f64 {
    /// # Examples
    /// ```
    /// fn test_one() {
    ///     use eng_fmt::FormatEng;
    ///     assert_eq!(
    ///         1_f64.format_eng(None),
    ///         String::from("001")
    ///     );
    /// }
    /// ```
    /// ```
    /// fn test_pi() {
    ///     use eng_fmt::FormatEng;
    ///     assert_eq!(
    ///         std::f64::consts::PI.format_eng(None),
    ///         String::from("3.14")
    ///     );
    /// }
    /// ```
    /// ```
    /// fn test_pi_5d() {
    ///     use eng_fmt::FormatEng;
    ///     assert_eq!(
    ///         std::f64::consts::PI.format_eng(Some(5)),
    ///         String::from("3.1415")
    ///     );
    /// }
    /// ```
    /// ```
    /// fn test_2pi_5d() {
    ///     use eng_fmt::FormatEng;
    ///     assert_eq!(
    ///         (std::f64::consts::PI * 2.).format_eng(Some(5)),
    ///         String::from("3.1415")
    ///     );
    /// }
    fn format_eng(&self, sf: Option<usize>) -> String {
        format_eng(*self, sf)
    }
}

/// Returns f64 as string in [engineering
/// notation](https://en.wikipedia.org/wiki/Engineering_notation) with last digit rounded to nearest
/// rather than truncated.
/// # Arguments
/// * x - value to be formatted
/// * s - number of significant figures, defaults to 3
/// ```
pub fn format_eng(x: f64, sf: Option<usize>) -> String {
    let sf = sf.unwrap_or(3);
    assert!(sf >= 1, "`format_eng` arg `sf` must be at least 1.");

    if x == 0. {
        return format!("{x:.*}", sf - 1);
    }

    let abs_log10 = x.abs().log10();

    let exp_sci: i32 = if abs_log10 >= 0. {
        abs_log10.floor()
    } else {
        abs_log10.ceil()
    } as i32;

    // engineering notation exponent
    let exp_eng: i32 = if abs_log10 >= 0. {
        exp_sci - abs_log10.floor() as i32 % 3
    } else {
        exp_sci - abs_log10.ceil() as i32 % 3 - 3
    };

    // number of digits left of decimal _after_ formatting for engineering notation, should never
    // exceed 3
    let n_left_of_dec: i32 = if abs_log10 > 0. {
        abs_log10.floor() as i32 % 3 + 1
    } else {
        3 + -(-abs_log10.ceil() as i32 % 3)
    };

    assert!(
        n_left_of_dec <= 3,
        "n_left_of_dec: {} exceeds 3",
        n_left_of_dec
    );

    let n_dec = sf as i32 - n_left_of_dec;

    let mut x_base = match exp_eng {
        // _ if exp_eng < 0 => ,
        _ if exp_eng.abs() <= 2 => x,
        _ => x / 10_f64.powi(exp_eng),
    };

    // round `x_base` as appropriate
    let exp = sf as i32 - n_left_of_dec;
    x_base = (x_base * 10_f64.powi(exp)).round() * 10_f64.powi(-exp);

    match exp_eng {
        _ if (0..=2).contains(&exp_eng) => format!("{x_base:.*}", n_dec.max(0) as usize),
        _ => format!("{x_base:.*}e{}", n_dec.max(0) as usize, exp_eng),
    }
}

#[allow(unused_macros)]
#[macro_export]
/// Generates a String similar to output of `dbg` but without printing.  
/// https://doc.rust-lang.org/src/std/macros.rs.html#340-362
macro_rules! format_dbg {
    () => {
        format!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        format!("[{}:{}] {} = {:#?}",
            file!(),
            line!(),
            stringify!($val),
            $val
        )
    };
    ($($val:expr),+ $(,)?) => {
        ($(format_dbg!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2() {
        assert_eq!(2_f64.format_eng(None), String::from("2.00"));
    }
    #[test]
    fn test_pi_div_10() {
        assert_eq!(
            (std::f64::consts::PI / 10.).format_eng(None),
            String::from("314e-3")
        );
    }
    #[test]
    fn test_pi_div_5() {
        assert_eq!(
            (std::f64::consts::PI / 5.).format_eng(Some(4)),
            String::from("628.3e-3")
        );
    }
    #[test]
    fn test_n_pi_div_10() {
        assert_eq!(
            (-std::f64::consts::PI / 10.).format_eng(None),
            String::from("-314e-3")
        );
    }
    #[test]
    fn test_pi_div_100() {
        assert_eq!(
            (std::f64::consts::PI / 10.).format_eng(None),
            String::from("314e-3")
        );
    }
    #[test]
    fn test_pi_div_1000() {
        assert_eq!(
            (std::f64::consts::PI / 10.).format_eng(None),
            String::from("314e-3")
        );
    }
    #[test]
    fn test_pi() {
        assert_eq!(std::f64::consts::PI.format_eng(None), String::from("3.14"));
    }
    #[test]
    fn test_33p333() {
        assert_eq!(33.333_f64.format_eng(Some(7)), String::from("33.33300"));
    }
    #[test]
    fn test_66p666() {
        assert_eq!(66.666_f64.format_eng(None), String::from("66.7"));
    }
    #[test]
    fn test_333p33() {
        assert_eq!(333.33_f64.format_eng(None), String::from("333"));
    }
    #[test]
    fn test_666p66() {
        assert_eq!(666.66_f64.format_eng(None), String::from("667"));
    }
    #[test]
    fn test_3p3333e3() {
        assert_eq!(3.3333e3_f64.format_eng(None), String::from("3.33e3"));
    }
    #[test]
    fn test_6p6666e3() {
        assert_eq!(6.6666e3_f64.format_eng(None), String::from("6.67e3"));
    }
    #[test]
    fn test_33p333e6() {
        assert_eq!(33.333e6_f64.format_eng(None), String::from("33.3e6"));
    }
    #[test]
    fn test_66p666e6() {
        assert_eq!(66.666e6_f64.format_eng(None), String::from("66.7e6"));
    }
    #[test]
    fn test_2pi_5d() {
        assert_eq!(
            (std::f64::consts::PI * 2.).format_eng(Some(5)),
            String::from("6.2832")
        );
    }
    #[test]
    fn test_n2pi_5d() {
        assert_eq!(
            (-std::f64::consts::PI * 2.).format_eng(Some(5)),
            String::from("-6.2832")
        );
    }
    #[test]
    fn test_n2pi_2d() {
        assert_eq!(
            (-std::f64::consts::PI * 2.).format_eng(Some(2)),
            String::from("-6.3")
        );
    }
    #[test]
    fn test_n2pi_e5_2d() {
        assert_eq!(
            (-std::f64::consts::PI * 2. * 10_f64.powi(5)).format_eng(Some(2)),
            String::from("-630e3")
        );
    }
    #[test]
    fn test_2pi_e5_1d() {
        assert_eq!(
            (std::f64::consts::PI * 2. * 10_f64.powi(5)).format_eng(Some(1)),
            String::from("600e3")
        );
    }
    #[test]
    fn test_pi_1d() {
        assert_eq!(
            (std::f64::consts::PI).format_eng(Some(1)),
            String::from("3")
        );
    }
    #[test]
    fn test_zero_5sf() {
        assert_eq!(0_f64.format_eng(Some(5)), String::from("0.0000"));
    }
    #[test]
    fn test_zero() {
        assert_eq!(0_f64.format_eng(None), String::from("0.00"));
    }
}
