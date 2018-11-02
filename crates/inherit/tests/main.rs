#[macro_use]
extern crate inherit;
pub trait Node {
    fn loc(&self) -> SourceLocation;
    fn kind(&self) -> NodeKind;
}
pub enum NodeKind {
    Thing,
}
#[derive(Clone)]
pub struct SourceLocation;

#[inherit(Node)]
pub struct Thing {}
