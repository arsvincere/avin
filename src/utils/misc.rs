static POW_VEC: &'static [f64] = &[
    0.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
];
pub fn round(num: f64, precision: u8) -> f64 {
    assert!(precision <= 9);

    if precision == 0 {
        num.round()
    } else {
        let multiplier = POW_VEC[precision as usize];
        let tmp_value = (num * multiplier).round().abs() as u64;

        (tmp_value as f64 / multiplier) * num.signum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_round() {
        let x: f64 = 123.456789;

        assert_eq!(round(x, 1), 123.5);
        assert_eq!(round(x, 2), 123.46);
        assert_eq!(round(x, 3), 123.457);
        assert_eq!(round(x, 4), 123.4568);
        assert_eq!(round(x, 5), 123.45679);
        assert_eq!(round(x, 6), 123.456789);

        assert_eq!(round(123.9_f64, 0), 124.0);
    }

    #[test]
    fn l_round() {
        let x: f64 = 123.111111111;

        assert_eq!(round(x, 0), 123.0);
        assert_eq!(round(x, 1), 123.1);
        assert_eq!(round(x, 2), 123.11);
        assert_eq!(round(x, 3), 123.111);
        assert_eq!(round(x, 4), 123.1111);
        assert_eq!(round(x, 5), 123.11111);
        assert_eq!(round(x, 6), 123.111111);
        assert_eq!(round(x, 7), 123.1111111);
        assert_eq!(round(x, 8), 123.11111111);
        assert_eq!(round(x, 9), 123.111111111);
    }
}
