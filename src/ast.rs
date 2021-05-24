use crate::ast::Terminal::{Id, Hole};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Definitions {
    Defs(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Leaf(Terminal),
    Op(Terminal, Vec<Expression>),
}

impl Expression {
    pub fn terminals(&self) -> Vec<&Terminal> {
        match self {
            Expression::Leaf(t) => vec![t],
            Expression::Op(t, exps) => {
                let mut res = exps.iter()
                    .flat_map(|x| x.terminals()).collect::<Vec<&Terminal>>();
                res.insert(0, t);
                res
            }
        }
    }

    pub fn holes(&self) -> Vec<&Terminal> {
        self.terminals().iter().filter_map(|x| { match x {
            Id(_, _) => None,
            Hole(_, _) => Some(*x)
        }
        }).collect::<Vec<&Terminal>>()
    }

    pub fn to_sexp_string(&self) -> String {
        match self {
            Expression::Leaf(t) => { t.to_string() }
            Expression::Op(x, y) => {
                format!("({} {})",
                        x.to_string(),
                        y.iter().map(|x| x.to_sexp_string()).intersperse(" ".to_string()).collect::<String>()
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatementOp {
    DirectionalRewrite,
    BidirectionalRewrite,
    DiffApply,
}

#[derive(Debug, Clone)]
pub enum Rewrite {
    /// Precondition - Source - Destination - Conditions
    DRewrite(Option<Expression>, Expression, Expression, Vec<(Terminal, Expression)>),
    BRewrite(Option<Expression>, Expression, Expression, Vec<(Terminal, Expression)>),
    /// Formarly known as diff applier
    AddSearcher(Option<Expression>, Expression, Expression, Vec<(Terminal, Expression)>),
}

pub type Parameter = (Identifier, Annotation);
pub type Constructor = (Identifier, Vec<Parameter>);

#[derive(Debug, Clone)]
pub enum Statement {
    /// Name and rewrite definition
    RewriteDef(String, Rewrite),
    /// Name - Params - Return type - Body
    Function(String, Vec<Parameter>, Annotation, Option<Expression>),
    /// Name - Type params - Constructors
    Datatype(String, Vec<Identifier>, Vec<Constructor>),
    /// Equality of two expressions with possible precondition
    Goal(Option<Expression>, Expression, Expression)
 }

pub type Identifier = String;

#[derive(Debug, Clone)]
pub enum Terminal {
    Id(Identifier, Option<Box<Annotation>>),
    Hole(Identifier, Option<Box<Annotation>>)
}

impl ToString for Terminal {
    fn to_string(&self) -> String {
        match self {
            Id(x, a) => x.to_string(),
            Hole(x, a) => format!("?{}", x.to_string())
        }
    }
}

impl Terminal {
    pub fn ident(&self) -> &String {
        match self {
            Id(i, _) => {i}
            Hole(i, _) => {i}
        }
    }

    pub fn is_hole(&self) -> bool {
        match self {
            Id(i, _) => {false}
            Hole(i, _) => {true}
        }
    }

    pub fn is_id(&self) -> bool {
        !self.is_hole()
    }
}

#[derive(Debug, Clone)]
pub enum Annotation {
    Type(Expression),
    Placeholder(usize),
    MultiAnnot(Vec<Annotation>),
}

impl Annotation {
    pub fn has_type(&self) -> bool {
        self.get_type().is_some()
    }

    pub fn get_type(&self) -> Option<Expression> {
        match self {
            Annotation::Type(x) => Some(x.clone()),
            Annotation::Placeholder(_) => None,
            Annotation::MultiAnnot(x) => x.iter().find_map(|c| c.get_type())
        }
    }
}