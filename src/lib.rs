#[macro_use]
extern crate inherit;

pub mod expression;
pub mod module;
pub mod node;
pub mod statement;

use expression::{Expression, Identifier, Literal, PropertyKind};
use module::ModuleDeclaration;
use node::{Node, NodeKind, SourceLocation};
use statement::{FunctionBody, Statement};

#[inherit(Node)]
#[derive(Debug)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<ProgramPart>,
}
#[derive(Debug)]
pub enum SourceType {
    Script,
    Module,
}
#[derive(Debug)]
pub enum ProgramPart {
    Directive(Directive),
    Statement(Statement),
    ModuleDecl(ModuleDeclaration),
}
#[derive(Debug)]
pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    pub generator: bool,
    pub loc: SourceLocation,
}
#[derive(Debug)]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
    pub loc: SourceLocation,
}

impl Node for Directive {
    fn loc(&self) -> SourceLocation {
        self.loc.clone()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::ExpressionStatement
    }
}
#[derive(Debug)]
pub struct AssignmentProperty {
    pub key: Expression,
    pub value: Pattern,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}

#[derive(Debug)]
pub enum Pattern {
    Ident(Identifier),
    Object(Object),
    Array(Array),
    RestElement(Box<RestElement>),
    Assignment(Box<Assignment>),
}
#[inherit(Pattern)]
#[derive(Debug)]
pub struct Object {
    pub properties: Vec<AssignmentProperty>,
}
#[inherit(Pattern)]
#[derive(Debug)]
pub struct Array {
    pub elements: Vec<Option<Pattern>>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct RestElement {
    pub argument: Pattern,
}
#[inherit(Pattern)]
#[derive(Debug)]
pub struct Assignment {
    pub left: Pattern,
    pub right: Expression,
}

impl Node for Pattern {
    fn loc(&self) -> SourceLocation {
        match self {
            Pattern::Ident(ref i) => i.loc(),
            Pattern::Object(ref o) => o.loc(),
            Pattern::Array(ref a) => a.loc(),
            Pattern::RestElement(ref r) => r.loc(),
            Pattern::Assignment(ref a) => a.loc(),
        }
    }

    fn kind(&self) -> NodeKind {
        match self {
            Pattern::Ident(ref i) => i.kind(),
            Pattern::Object(ref o) => o.kind(),
            Pattern::Array(ref a) => a.kind(),
            Pattern::RestElement(ref r) => r.kind(),
            Pattern::Assignment(ref a) => a.kind(),
        }
    }
}
#[derive(Debug)]
pub struct Class {
    pub id: Option<Identifier>,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
    pub loc: SourceLocation,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct ClassBody {
    pub body: Vec<MethodDefinition>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct MethodDefinition {
    pub key: Expression,
    //TODO: FunctionExpression?
    pub value: Function,
    pub kind: MethodKind,
    pub computed: bool,
    pub _static: bool,
}
#[derive(Debug)]
pub enum MethodKind {
    Constructor,
    Method,
    Get,
    Set,
}
