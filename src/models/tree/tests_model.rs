// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use dedent::dedent;
use std::vec;

use super::base::GenericTree;
use super::base::GenericTreeLike;
use super::base::GenericTreeOrRoot;

// ----------------------------------------------------------------
// TESTS
// ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let t = GenericTree::new(
            DummyNode {
                name: Some("root".to_string()),
                value: None,
            },
            None,
        );
        assert_eq!(t.to_string(), "root");
    }

    #[test]
    fn test_case2() {
        let mut t = GenericTree::new(
            DummyNode {
                name: Some("root".to_string()),
                value: None,
            },
            None,
        );
        let child1 = DummyNode {
            name: Some("alice".to_string()),
            value: Some(23),
        };
        let child2 = DummyNode {
            name: Some("bob".to_string()),
            value: Some(24),
        };
        t.add(GenericTreeOrRoot::Root(child1));
        t.add(GenericTreeOrRoot::Root(child2));
        let expected = dedent!(
            r#"
            root
            ├─── alice: 23
            ╰─── bob: 24
        "#
        )
        .to_string();
        assert_eq!(t.to_string(), expected);
    }

    #[test]
    fn test_case3() {
        let mut t = GenericTree::new(
            DummyNode {
                name: Some("root".to_string()),
                value: None,
            },
            None,
        );
        let mut child1 = GenericTree::new(
            DummyNode {
                name: Some("alice".to_string()),
                value: Some(23),
            },
            None,
        );
        let pet1a = DummyNode {
            name: Some("bird".to_string()),
            value: Some(2),
        };
        let pet1b = DummyNode {
            name: None,
            value: Some(3),
        };
        child1.add(GenericTreeOrRoot::Root(pet1a));
        child1.add(GenericTreeOrRoot::Root(pet1b));
        let child2 = DummyNode {
            name: Some("bob".to_string()),
            value: Some(24),
        };
        t.add(GenericTreeOrRoot::Tree(child1));
        t.add(GenericTreeOrRoot::Root(child2));
        let expected = dedent!(
            r#"
            root
            ├──╮ alice: 23
            │  ├─── bird: 2
            │  ╰─── _: 3
            ╰─── bob: 24
        "#
        )
        .to_string();
        assert_eq!(t.to_string(), expected);
    }
}

// ----------------------------------------------------------------
// AUXILIARY
// ----------------------------------------------------------------

#[derive(Clone)]
struct DummyNode {
    name: Option<String>,
    value: Option<i32>,
}

impl ToString for DummyNode {
    fn to_string(&self) -> String {
        match (&self.name, &self.value) {
            (Some(name), Some(value)) => {
                format!("{}: {}", name, value)
            }
            (Some(name), None) => name.clone(),
            (None, Some(value)) => {
                format!("_: {}", value)
            }
            (None, None) => "-".to_string(),
        }
    }
}
