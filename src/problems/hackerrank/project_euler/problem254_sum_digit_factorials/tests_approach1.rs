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
        let args = vec![(3, 1000000), (20, 1000000)];
        let result = approach::run(&args);
        let mut expected = HashMap::<(i64, i64), i64>::new();
        expected.insert((3, 1000000), 8);
        expected.insert((20, 1000000), 156);
        assert_eq!(result, expected);
    }
}
