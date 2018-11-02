use crate::{
    expression::{Expression, Literal},
    statement::Declaration,
    Identifier, Node, NodeKind, SourceLocation,
};
#[derive(Debug)]
pub enum ModuleDeclaration {
    Import(ImportDeclaration),
    Export(ExportDeclaration),
}

impl Node for ModuleDeclaration {
    fn loc(&self) -> SourceLocation {
        match self {
            ModuleDeclaration::Import(ref inner) => inner.loc(),
            ModuleDeclaration::Export(ref inner) => inner.loc(),
        }
    }

    fn kind(&self) -> NodeKind {
        match self {
            ModuleDeclaration::Import(ref inner) => inner.kind(),
            ModuleDeclaration::Export(ref inner) => inner.kind(),
        }
    }
}

#[inherit(Node)]
#[derive(Debug)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
}
#[derive(Debug)]
pub enum ImportSpecifier {
    Named(Import),
    Default(ImportDefault),
    NameSpace(ImportNamespace),
}

#[inherit(ImportSpecifier)]
#[derive(Debug)]
pub struct Import {
    pub import: Identifier,
}
#[inherit(ImportSpecifier)]
#[derive(Debug)]
pub struct ImportDefault {}

#[inherit(ImportSpecifier)]
#[derive(Debug)]
pub struct ImportNamespace {}
#[derive(Debug)]
pub enum ExportDeclaration {
    Named(ExportNamed),
    Default(ExportDefault),
    All(ExportAll),
}

impl Node for ExportDeclaration {
    fn loc(&self) -> SourceLocation {
        match self {
            ExportDeclaration::Named(ref inner) => inner.loc(),
            ExportDeclaration::Default(ref inner) => inner.loc(),
            ExportDeclaration::All(ref inner) => inner.loc(),
        }
    }
    fn kind(&self) -> NodeKind {
        match self {
            ExportDeclaration::Named(ref inner) => inner.kind(),
            ExportDeclaration::Default(ref inner) => inner.kind(),
            ExportDeclaration::All(ref inner) => inner.kind(),
        }
    }
}

#[inherit(Declaration)]
#[derive(Debug)]
pub struct ExportNamed {
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
    pub exported: Identifier,
}
#[inherit(Node)]
#[derive(Debug)]
pub struct ExportSpecifier {
    pub exported: Identifier,
}

#[inherit(Declaration)]
#[derive(Debug)]
pub struct ExportDefault {
    pub declaration: ExportDecl,
}

#[inherit(Declaration)]
#[derive(Debug)]
pub struct ExportAll {
    pub source: Literal,
}
#[derive(Debug)]
pub enum ExportDecl {
    Decl(Declaration),
    Expr(Expression),
}
