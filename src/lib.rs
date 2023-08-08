//! Module containing trait to format f64 in [engineering
//! notation](https://en.wikipedia.org/wiki/Engineering_notation)

/// Trait providing method for formatting numbers in [engineering
/// notation](https://en.wikipedia.org/wiki/Engineering_notation)
pub trait FormatEng {
    fn format_eng(&self, sf: Option<usize>) -> String;
}

impl FormatEng for f64 {
    /// Returns f64 as string in [engineering
    /// notation](https://en.wikipedia.org/wiki/Engineering_notation) with last digit rounded to nearest
    /// rather than truncated.
    /// # Arguments
    /// * x - value to be formatted
    /// * s - number of significant figures, defaults to 3
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
    /// ```
    fn format_eng(&self, sf: Option<usize>) -> String {
        let s = sf.unwrap_or(3);

        if *self == 0. {
            return format!("{self:.*}", s - 1);
        }

        let abs_log10 = self.abs().log10();

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
        let n_left_of_dec: usize = if abs_log10 > 0. {
            abs_log10.floor() as usize % 3 + 1
        } else {
            3 + -(-abs_log10.ceil() as i32 % 3) as usize
        };

        assert!(
            n_left_of_dec <= 3,
            "n_left_of_dec: {} exceeds 3",
            n_left_of_dec
        );

        let n_dec = s - n_left_of_dec;

        let mut x_base = match exp_eng {
            // _ if exp_eng < 0 => ,
            _ if exp_eng.abs() <= 2 => *self,
            _ => self / 10_f64.powi(exp_eng),
        };

        // round `x_base` as appropriate
        x_base = match n_left_of_dec {
            // the other branch should work for this first case, but doing this separately should be
            // more cpu efficient
            _ if n_left_of_dec == 3 && exp_eng > 0 => x_base.round(),
            _ => {
                (x_base * 10_f64.powi((s - n_left_of_dec) as i32)).round()
                    * 10_f64.powf(-((s - n_left_of_dec) as f64))
            }
        };

        match exp_eng {
            _ if (0..=2).contains(&exp_eng) => format!("{x_base:.*}", n_dec),
            _ => format!("{x_base:.*}e{}", n_dec, exp_eng),
        }
    }
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
    fn test_zero_5sf() {
        assert_eq!(0_f64.format_eng(Some(5)), String::from("0.0000"));
    }
    #[test]
    fn test_zero() {
        assert_eq!(0_f64.format_eng(None), String::from("0.00"));
    }
}
