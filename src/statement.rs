use crate::{
    expression::Expression, Class, Directive, Function, Identifier, Node, NodeKind, Pattern,
    SourceLocation,
};
#[derive(Debug)]
pub enum Statement {
    Expr(Expression),
    Block(Block),
    FuncBody(FunctionBody),
    Empty(Empty),
    Debugger(Debugger),
    With(Box<With>),
    Return(Return),
    Labeled(Box<Labeled>),
    Break(Break),
    Continue(Continue),
    If(Box<If>),
    Switch(Switch),
    Throw(Throw),
    Try(Try),
    While(Box<While>),
    DoWhile(Box<DoWhile>),
    For(Box<For>),
    ForIn(Box<ForIn>),
    Decl(Declaration),
}

impl Node for Statement {
    fn loc(&self) -> SourceLocation {
        match self {
            Statement::Expr(ref inner) => inner.loc(),
            Statement::Block(ref inner) => inner.loc(),
            Statement::FuncBody(ref inner) => inner.loc(),
            Statement::Empty(ref inner) => inner.loc(),
            Statement::Debugger(ref inner) => inner.loc(),
            Statement::With(ref inner) => inner.loc(),
            Statement::Return(ref inner) => inner.loc(),
            Statement::Labeled(ref inner) => inner.loc(),
            Statement::Break(ref inner) => inner.loc(),
            Statement::Continue(ref inner) => inner.loc(),
            Statement::If(ref inner) => inner.loc(),
            Statement::Switch(ref inner) => inner.loc(),
            Statement::Throw(ref inner) => inner.loc(),
            Statement::Try(ref inner) => inner.loc(),
            Statement::While(ref inner) => inner.loc(),
            Statement::DoWhile(ref inner) => inner.loc(),
            Statement::For(ref inner) => inner.loc(),
            Statement::ForIn(ref inner) => inner.loc(),
            Statement::Decl(ref inner) => inner.loc(),
        }
    }

    fn kind(&self) -> NodeKind {
        match self {
            Statement::Expr(_) => NodeKind::ExpressionStatement,
            Statement::Block(ref inner) => inner.kind(),
            Statement::FuncBody(ref inner) => inner.kind(),
            Statement::Empty(ref inner) => inner.kind(),
            Statement::Debugger(ref inner) => inner.kind(),
            Statement::With(ref inner) => inner.kind(),
            Statement::Return(ref inner) => inner.kind(),
            Statement::Labeled(ref inner) => inner.kind(),
            Statement::Break(ref inner) => inner.kind(),
            Statement::Continue(ref inner) => inner.kind(),
            Statement::If(ref inner) => inner.kind(),
            Statement::Switch(ref inner) => inner.kind(),
            Statement::Throw(ref inner) => inner.kind(),
            Statement::Try(ref inner) => inner.kind(),
            Statement::While(ref inner) => inner.kind(),
            Statement::DoWhile(ref inner) => inner.kind(),
            Statement::For(ref inner) => inner.kind(),
            Statement::ForIn(ref inner) => inner.kind(),
            Statement::Decl(ref inner) => inner.kind(),
        }
    }
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Block {
    pub body: Vec<Statement>,
}
#[derive(Debug)]
pub struct FunctionBody {
    pub body: Vec<FunctionBodyPart>,
    pub loc: SourceLocation,
}
impl Node for FunctionBody {
    fn loc(&self) -> SourceLocation {
        self.loc.clone()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::BlockStatement
    }
}
#[derive(Debug)]
pub enum FunctionBodyPart {
    Directive(Directive),
    Statement(Statement),
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Empty {}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Debugger {}
#[inherit(Statement)]
#[derive(Debug)]
pub struct With {
    pub object: Expression,
    pub body: Statement,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Return {
    pub argument: Option<Expression>,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Labeled {
    pub label: Identifier,
    pub body: Statement,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Break {
    pub label: Option<Identifier>,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Continue {
    pub label: Option<Identifier>,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct If {
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Switch {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Throw {
    pub argument: Expression,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct Try {
    pub block: Block,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<Block>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct CatchClause {
    pub param: Pattern,
    pub body: Block,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct While {
    pub test: Expression,
    pub body: Statement,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct DoWhile {
    pub body: Statement,
    pub test: Expression,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct For {
    pub init: ForInit,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}
#[derive(Debug)]
pub enum ForInit {
    Var(VariableDeclaration),
    Expr(Expression),
    Null,
}
#[inherit(Statement)]
#[derive(Debug)]
pub struct ForIn {
    pub left: ForLeft,
    pub right: Expression,
    pub body: Statement,
}
#[derive(Debug)]
pub enum ForLeft {
    Var(VariableDeclaration),
    Pat(Pattern),
}
#[derive(Debug)]
pub enum Declaration {
    Function(Function),
    Variable(VariableDeclaration),
    Class(Class),
}

impl Node for Declaration {
    fn loc(&self) -> SourceLocation {
        match self {
            Declaration::Function(ref inner) => inner.loc.clone(),
            Declaration::Variable(ref inner) => inner.loc(),
            Declaration::Class(ref inner) => inner.loc.clone(),
        }
    }

    fn kind(&self) -> NodeKind {
        match self {
            Declaration::Function(_) => NodeKind::FunctionDeclaration,
            Declaration::Class(_) => NodeKind::ClassDeclaration,
            Declaration::Variable(ref inner) => inner.kind(),
        }
    }
}
#[inherit(Node)]
#[derive(Debug)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}
#[derive(Debug)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}
