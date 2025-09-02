/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::collections::HashMap;

use super::approach1 as approach;

/// ----------------------------------------------------------------
/// TESTS
/// ----------------------------------------------------------------

/// bundle of tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let args = vec![5, 10];
        let result = approach::run(&args);
        let mut expected = HashMap::<i64, i64>::new();
        expected.insert(5, 10);
        expected.insert(10, 17);
        assert_eq!(result, expected);
    }
}
