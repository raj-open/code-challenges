/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

///

/// ----------------------------------------------------------------
/// TRAITS
/// ----------------------------------------------------------------

/// Struct to handle generic trees
#[allow(unused)]
pub struct GenericTree<T> {
    pub root: T,
    pub children: Vec<GenericTree<T>>,
}

#[allow(unused)]
pub enum GenericTreeOrRoot<T> {
    Root(T),
    Tree(GenericTree<T>),
}

/// Interface for handling generic trees
#[allow(unused)]
pub trait GenericTreeLike<T> {
    /// Adds another tree (or node) as a child
    fn add(&mut self, other: GenericTreeOrRoot<T>);

    /// Displays a single node
    fn repr_node(
        node: &T,
        indent: Option<&str>,
        sep: Option<&str>,
        lex: Option<&mut Vec<bool>>,
    ) -> String;

    /// Representation of the entire tree
    fn repr_tree(
        &self,
        indent: Option<&str>,
        sep: Option<&str>,
        lex: Option<&Vec<bool>>,
    ) -> Vec<String>;
}
