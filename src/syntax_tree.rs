use std::fmt::{Display, Formatter};

pub type ID = usize;

static mut LAST_ID: usize = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTree<T> {
    id: ID,
    value: T,
    children: Vec<SyntaxTree<T>>,
}

/// Simple ID provider
fn next_id() -> ID {
    unsafe {
        let id = LAST_ID;
        LAST_ID += 1;
        id
    }
}

impl<T> SyntaxTree<T> {
    /// Create a SyntaxTree with a root node that carries the given value
    pub fn new(value: T) -> SyntaxTree<T> {
        todo!()
    }

    /// Add another SyntaxTree as last child of this tree
    pub fn push_node(&mut self, new_node: SyntaxTree<T>) {
        todo!()
    }

    /// Create a new SyntaxTree with a root node that carries the given value. Add the created tree
    /// as last child of this tree.
    pub fn push_value(&mut self, value: T) {
        self.push_node(SyntaxTree::new(value));
    }

    /// Add another SyntaxTree as first child of this tree
    pub fn prepend_node(&mut self, new_node: SyntaxTree<T>) {
        todo!()
    }

    /// Create a new SyntaxTree with a root node that carries the given value. Add the created tree
    /// as first child of this tree.
    pub fn prepend_value(&mut self, value: T) {
        self.prepend_node(SyntaxTree::new(value));
    }

    /// Insert the given SyntaxTree into the children of this tree at the given index
    pub fn insert_node(&mut self, index: usize, new_node: SyntaxTree<T>) {
        self.children.insert(index, new_node);
    }

    /// Create a new SyntaxTree with a root node that carries the given value.
    /// Insert the created SyntaxTree into the children of this tree at the given index
    pub fn insert_value(&mut self, index: usize, value: T) {
        self.insert_node(index, SyntaxTree::new(value));
    }

    /// Perform a depth-first search with the given predicate.
    /// The method returns a reference to the first SyntaxTree instance for which the predicate
    /// return true. If no instance is found, None is returned.
    pub fn find_node(&self, predicate: fn(&SyntaxTree<T>) -> bool) -> Option<&SyntaxTree<T>> {
        if predicate(self) {
            Some(self)
        } else {
            todo!()
        }
    }

    /// Perform a depth-first search with the given predicate.
    /// The method returns a mutable reference to the first SyntaxTree instance for which the predicate
    /// return true. If no instance is found, None is returned.
    pub fn find_node_mut(
        &mut self,
        predicate: fn(&SyntaxTree<T>) -> bool,
    ) -> Option<&SyntaxTree<T>> {
        todo!()
    }

    /// Return a reference to the value carried by the root of this tree
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Return the id of the root of this tree
    pub fn id(&self) -> ID {
        self.id
    }

    /// Return a reference to the children of this tree
    pub fn children(&self) -> &Vec<SyntaxTree<T>> {
        &self.children
    }
}

impl<T: Display> SyntaxTree<T> {
    pub fn print(&self) -> String {
        if self.children.is_empty() {
            format!("{}", self.value)
        } else {
            format!(
                "{}\n[\n{}\n]",
                self.value,
                &self
                    .children
                    .iter()
                    .map(|tn| tn.print_inner(1))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )
        }
    }

    pub fn print_inner(&self, indent: usize) -> String {
        let mut indentation = String::new();
        for _ in 0..indent {
            indentation.push_str("  ");
        }
        if self.children.is_empty() {
            format!("{}{}", &indentation, self.value)
        } else {
            format!(
                "{}{}\n{}[\n{}\n{}]",
                &indentation,
                self.value,
                &indentation,
                &self
                    .children
                    .iter()
                    .map(|tn| tn.print_inner(indent + 1))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                indentation,
            )
        }
    }
}

impl<T: Display> Display for SyntaxTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.print())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_tree_numbers() -> SyntaxTree<i32> {
        let mut tree = SyntaxTree::new(0);

        for child in 1..3 {
            let mut child = SyntaxTree::new(child);
            for grandchild in 1..3 {
                let id = grandchild * 10;
                child.prepend_node(SyntaxTree::new(id));
            }
            tree.push_node(child);
        }
        tree
    }

    fn fill_tree_words() -> SyntaxTree<String> {
        let mut tree = SyntaxTree::new(to_s("root"));

        for (child_id, child) in ["first", "second", "third"].iter().map(to_s).enumerate() {
            let child_id = child_id;
            let mut child = SyntaxTree::new(child);
            if child_id == 0 {
                let mut descendant1 = SyntaxTree::new(to_s("A"));
                let mut descendant2 = SyntaxTree::new(to_s("B"));
                let descendant3 = SyntaxTree::new(to_s("C"));
                descendant2.push_node(descendant3);
                descendant1.push_node(descendant2);
                child.push_node(descendant1);
            }
            tree.push_node(child);
        }
        tree
    }

    #[test]
    fn number_tree() -> Result<(), String> {
        let tree = fill_tree_numbers();

        println!("{}", tree);
        assert_eq!(
            String::from(
                "0\n[\n  1\n  [\n    20,\n    10\n  ],\n  2\n  [\n    20,\n    10\n  ]\n]"
            ),
            tree.print()
        );
        Ok(())
    }

    #[test]
    fn word_tree() -> Result<(), String> {
        let tree = fill_tree_words();

        println!("{}", tree);
        assert_eq!(
            String::from("root\n[\n  first\n  [\n    A\n    [\n      B\n      [\n        C\n      ]\n    ]\n  ],\n  second,\n  third\n]"),
            tree.print()
        );
        Ok(())
    }

    #[test]
    fn find_node_by_value() -> Result<(), String> {
        let tree = fill_tree_numbers();

        assert!(tree.find_node(|n| n.value == 0).is_some());
        let left = tree.find_node(|n| n.value == 1).unwrap();
        assert!(left.find_node(|n| n.value == 10).is_some());
        assert!(left.find_node(|n| n.value == 20).is_some());

        let right = tree.find_node(|n| n.value == 2).unwrap();
        assert!(right.find_node(|n| n.value == 10).is_some());
        assert!(right.find_node(|n| n.value == 20).is_some());
        Ok(())
    }

    fn to_s<T: Display>(value: T) -> String {
        format!("{}", value)
    }
}
