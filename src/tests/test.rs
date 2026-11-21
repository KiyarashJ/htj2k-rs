#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude_exponent() {
        assert_eq!(magnitude_exponent(0), 0);
        assert_eq!(magnitude_exponent(1), 1);
        assert_eq!(magnitude_exponent(2), 2);
        assert_eq!(magnitude_exponent(3), 3);
        assert_eq!(magnitude_exponent(4), 3);
        assert_eq!(magnitude_exponent(5), 4);
        assert_eq!(magnitude_exponent(8), 4);
        assert_eq!(magnitude_exponent(9), 5);
    }

    #[test]
    fn test_quad_analysis() {
        let quad = [3, 0, -2, 0];

        let result = analyze_quad(&quad);

        assert_eq!(result.magnitudes, [3, 0, 2, 0]);
        assert_eq!(result.exponents, [3, 0, 2, 0]);

        assert_eq!(result.rho, 5);
        assert_eq!(result.gamma, 1);

        assert_eq!(result.magsgn_values, [4, 0, 3, 0]);
    }
}