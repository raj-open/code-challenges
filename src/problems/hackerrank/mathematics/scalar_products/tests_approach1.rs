/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------
use crate::problems::hackerrank::mathematics::scalar_products::approach1 as approach;

/// ----------------------------------------------------------------
/// TESTS
/// ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = approach::run(4, 5, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_case2() {
        let result = approach::run(1, 100, 1000);
        assert_eq!(result, 50);
    }
}
