mod stack;
mod syntax_tree;

pub trait Stack {
    fn init() -> Self;

    fn push_val(&mut self, i: i32);

    fn top_val(&self) -> Option<&i32>;

    fn pop_val(&mut self) -> Option<i32>;

    fn is_empty(&self) -> bool;
}

pub use stack::ListStack;
pub use syntax_tree::SyntaxTree;
pub use syntax_tree::ID;

#[cfg(test)]
mod tests {}
