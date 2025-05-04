/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::vec;

use super::base::GenericTree;
use super::base::GenericTreeLike;
use super::base::GenericTreeOrRoot;

/// ----------------------------------------------------------------
/// BASIC IMPLEMENTATION FOR GenericTree
/// ----------------------------------------------------------------

#[allow(unused)]
impl<T> GenericTree<T>
where
    T: Clone,
{
    pub fn new(root: T, children: Option<Vec<GenericTree<T>>>) -> Self {
        let children = children.map_or_else(|| vec![], |elements| elements.clone());
        return Self { root, children };
    }
}

impl<T> GenericTree<T> {
    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    pub fn has_children(&self) -> bool {
        self.num_children() > 0
    }
}

impl<T> Clone for GenericTree<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let root = self.root.clone();
        let children: Vec<GenericTree<T>> = self.children.iter().map(|u| u.clone()).collect();
        return Self { root, children };
    }
}

impl<T> ToString for GenericTree<T>
where
    GenericTree<T>: GenericTreeLike<T>,
{
    fn to_string(&self) -> String {
        let lines = self.repr_tree(None, None, None);
        let repr = lines.join("\n");
        return repr;
    }
}

/// ----------------------------------------------------------------
/// BASIC IMPLEMENTATION OF TRAIT FOR GenericTree
/// ----------------------------------------------------------------

impl<T> GenericTreeLike<T> for GenericTree<T>
where
    T: ToString,
{
    fn add(&mut self, other: GenericTreeOrRoot<T>) {
        match other {
            GenericTreeOrRoot::Root(u) => {
                let t = GenericTree {
                    root: u,
                    children: vec![],
                };
                self.children.push(t);
            }
            GenericTreeOrRoot::Tree(t) => {
                self.children.push(t);
            }
        }
    }

    /// Displays a single node
    fn repr_node(
        node: &T,
        indent: Option<&str>,
        sep: Option<&str>,
        lex: Option<&mut Vec<bool>>,
    ) -> String {
        let indent: &str = indent.map_or_else(|| "  ", |x| x);
        let sep: &str = sep.map_or_else(|| "  ", |x| x);
        let lex: Vec<bool> = lex.map_or_else(|| vec![], |x| x.clone());
        let prefix;
        if lex.len() <= 1 {
            return node.to_string();
        } else {
            prefix = lex[..lex.len() - 1]
                .iter()
                .enumerate()
                .map(|(k, &is_last)| match (k, is_last) {
                    (0, _) => "".to_string(),
                    (_, false) => format!("│{}", indent),
                    (_, _) => format!(" {}", indent),
                })
                .collect::<Vec<String>>()
                .join("");
            return format!("{}{}{}", prefix, sep, node.to_string());
        }
    }

    /// Method to recursive display elements of Tree
    fn repr_tree(
        &self,
        indent: Option<&str>,
        sep: Option<&str>,
        lex: Option<&Vec<bool>>,
    ) -> Vec<String> {
        let indent: &str = indent.map_or_else(|| "  ", |x| x);
        let sep: &str = sep.map_or_else(|| "  ", |x| x);
        let mut lex: Vec<bool> = lex.map_or_else(|| vec![true], |x| x.clone());
        let mut result: Vec<String> = vec![];

        let line = Self::repr_node(&self.root, Some(indent), Some(sep), Some(&mut lex));
        result.push(line);

        let n = self.num_children();
        for (k, child) in self.children.iter().enumerate() {
            let is_final = k == n - 1;
            let connector = if child.has_children() { "╮ " } else { "─ " };
            let sep_ = if is_final {
                format!("╰──{}", connector)
            } else {
                format!("├──{}", connector)
            };
            let mut lex_ = lex.clone();
            lex_.push(is_final);
            let mut result_ = child.repr_tree(Some(indent), Some(sep_.as_str()), Some(&lex_));
            result.append(&mut result_);
        }

        return result;
    }
}
