use crate::{Class, Function, FunctionBody, Node, NodeKind, Pattern, SourceLocation};
#[derive(Debug)]
pub enum Expression {
    This(This),
    Array(Array),
    Object(Object),
    Function(Box<Function>),
    Unary(Box<Unary>),
    Update(Box<Update>),
    Binary(Box<Binary>),
    Assignment(Box<Assignment>),
    Logical(Box<Logical>),
    Member(Box<Member>),
    Conditional(Box<Conditional>),
    Call(Box<Call>),
    New(Box<New>),
    Sequence(Sequence),
    Spread(Box<Expression>),
    ArrowFunction(Box<ArrowFunction>),
    Yield(Box<Expression>),
    Await(Box<Expression>),
    Identifier(Box<Identifier>),
    Literal(Box<Literal>),
    TaggedTemplate(Box<TaggedTemplate>),
    Class(Box<Class>),
    Meta(MetaProperty),
}

impl Node for Expression {
    fn loc(&self) -> SourceLocation {
        match self {
            Expression::This(ref inner) => inner.loc(),
            Expression::Array(ref inner) => inner.loc(),
            Expression::Object(ref inner) => inner.loc(),
            Expression::Function(ref inner) => inner.loc.clone(),
            Expression::Unary(ref inner) => inner.loc(),
            Expression::Update(ref inner) => inner.loc(),
            Expression::Binary(ref inner) => inner.loc(),
            Expression::Assignment(ref inner) => inner.loc(),
            Expression::Logical(ref inner) => inner.loc(),
            Expression::Member(ref inner) => inner.loc(),
            Expression::Conditional(ref inner) => inner.loc(),
            Expression::Call(ref inner) => inner.loc(),
            Expression::New(ref inner) => inner.loc(),
            Expression::Sequence(ref inner) => inner.loc(),
            Expression::Spread(ref inner) => inner.loc(),
            Expression::Yield(ref inner) => inner.loc(),
            Expression::Await(ref inner) => inner.loc(),
            Expression::Identifier(ref inner) => inner.loc(),
            Expression::Literal(ref inner) => inner.loc(),
            Expression::ArrowFunction(ref inner) => inner.loc(),
            Expression::TaggedTemplate(ref inner) => inner.loc(),
            Expression::Class(ref inner) => inner.loc.clone(),
            Expression::Meta(ref inner) => inner.loc(),
        }
    }
    fn kind(&self) -> NodeKind {
        match self {
            Expression::This(ref inner) => inner.kind(),
            Expression::Array(ref inner) => inner.kind(),
            Expression::Object(ref inner) => inner.kind(),
            Expression::Function(_) => NodeKind::FunctionExpression,
            Expression::Unary(ref inner) => inner.kind(),
            Expression::Update(ref inner) => inner.kind(),
            Expression::Binary(ref inner) => inner.kind(),
            Expression::Assignment(ref inner) => inner.kind(),
            Expression::Logical(ref inner) => inner.kind(),
            Expression::Member(ref inner) => inner.kind(),
            Expression::Conditional(ref inner) => inner.kind(),
            Expression::Call(ref inner) => inner.kind(),
            Expression::New(ref inner) => inner.kind(),
            Expression::Sequence(ref inner) => inner.kind(),
            Expression::Spread(ref inner) => inner.kind(),
            Expression::Yield(ref inner) => inner.kind(),
            Expression::Await(ref inner) => inner.kind(),
            Expression::Identifier(ref inner) => inner.kind(),
            Expression::Literal(ref inner) => inner.kind(),
            Expression::ArrowFunction(ref inner) => inner.kind(),
            Expression::TaggedTemplate(ref inner) => inner.kind(),
            Expression::Class(_) => NodeKind::ClassExpression,
            Expression::Meta(ref inner) => inner.kind(),
        }
    }
}
#[inherit(Node)]
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct Literal {
    pub value: LiteralValue,
}
#[derive(Debug)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(String),
    RegExp(RegExpLiteral),
    TemplateLiteral(TemplateLiteral),
}
#[derive(Debug)]
pub struct RegExpLiteral {
    pub regex: RegEx,
    pub loc: SourceLocation,
}

impl Node for RegExpLiteral {
    fn loc(&self) -> SourceLocation {
        self.loc.clone()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Literal
    }
}
#[derive(Debug)]
pub struct RegEx {
    pub pattern: String,
    pub flags: String,
    pub loc: SourceLocation,
}

impl Node for RegEx {
    fn loc(&self) -> SourceLocation {
        self.loc.clone()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Literal
    }
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct This {
    // pub type: "ThisExpression"
}
#[inherit(Node)]
#[derive(Debug)]
pub struct Super {}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Array {
    pub elements: Vec<Option<Spreadable>>,
}
#[derive(Debug)]
pub enum Spreadable {
    Expr(Expression),
    Spread(SpreadElement),
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Object {
    pub properties: Vec<Property>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct Property {
    pub key: Expression,
    pub value: Expression,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}
#[derive(Debug)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Expression,
}
#[derive(Debug)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
    Tilde,
    TypeOf,
    Void,
    Delete,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Update {
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
}

#[derive(Debug)]
pub enum UpdateOperator {
    Decrement,
    Increment,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}
#[derive(Debug)]
pub enum BinaryOperator {
    And,
    GreaterThan,
    GreaterThanEqual,
    Div,
    Equal,
    In,
    InstanceOf,
    LeftShift,
    LessThan,
    LessThanEqual,
    Minus,
    Mod,
    NotEqual,
    Or,
    Plus,
    RightShift,
    StrictEqual,
    StrictNotEqual,
    Times,
    UnsignedRightShift,
    XOr,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Assignment {
    pub operator: AssignmentOperator,
    pub left: PatOrExpr,
    pub right: Expression,
}
#[derive(Debug)]
pub enum PatOrExpr {
    Pat(Pattern),
    Expr(Expression),
}
#[derive(Debug)]
pub enum AssignmentOperator {
    AddAssign,
    AndAssign,
    Assign,
    DivAssign,
    LHSAssign,
    MulAssign,
    ModAssign,
    OrAssign,
    RHSAssign,
    SubAssign,
    URHSAssign,
    XOrAssign,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Logical {
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}
#[derive(Debug)]
pub enum LogicalOperator {
    Or,
    And,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Member {
    pub object: Parent,
    pub property: Expression,
    pub computed: bool,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Conditional {
    pub test: Expression,
    pub alternate: Expression,
    pub consequent: Expression,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Call {
    pub callee: Parent,
    pub arguments: Vec<Spreadable>,
}
#[derive(Debug)]
pub enum Parent {
    Expr(Expression),
    Super(Super),
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct New {
    pub callee: Expression,
    pub arguments: Vec<Spreadable>,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Sequence {
    pub expressions: Vec<Expression>,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct SpreadElement {
    pub argument: Expression,
}
#[inherit(Function, Expression)]
#[derive(Debug)]
pub struct ArrowFunction {
    pub body: ArrowFunctionBody,
}
#[derive(Debug)]
pub enum ArrowFunctionBody {
    Expr(Expression),
    Block(FunctionBody),
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct Yield {
    pub argument: Option<Expression>,
    pub delegate: bool,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expression>,
}
#[inherit(Expression)]
#[derive(Debug)]
pub struct TaggedTemplate {
    pub tag: Expression,
    pub quasi: TemplateLiteral,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct TemplateElement {
    pub tail: bool,
    pub value: TemplateValue,
}
#[derive(Debug)]
pub struct TemplateValue {
    pub cooked: String,
    pub raw: String,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct MetaProperty {
    pub meta: Identifier,
    pub property: Identifier,
}
