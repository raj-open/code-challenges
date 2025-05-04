/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use super::approach2 as approach;

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

    #[test]
    // #[ignore]
    fn test_case_heavy_1() {
        let result = approach::run(991, 11495481, 112259);
        assert_eq!(result, 224515);
    }
}
