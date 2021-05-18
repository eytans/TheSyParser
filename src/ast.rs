use crate::ast::Terminal::{Id, Hole};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub enum Expression {
    Leaf(Terminal),
    Op(Terminal, Vec<Expression>),
}

impl Expression {
    fn terminals(&self) -> Vec<&Terminal> {
        match self {
            Expression::Leaf(t) => vec![t],
            Expression::Op(t, exps) =>
                vec![t].iter().chain(exps.iter())
                    .flat_map(|x| x.terminals())
                    .collect_vec()
        }
    }

    fn holes(&self) -> Vec<&Terminal> {
        self.terminals().iter().filter_map(|x| { match x {
            Id(_, _) => None,
            Hole(_, _) => Some(x)
        }
        }).collect_vec()
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum StatementOp {
    DirectionalRewrite,
    BidirectionalRewrite,
    DiffApply,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Rewrite {
    /// Precondition - Source - Destination - Conditions
    DRewrite(Option<Expression>, Expression, Expression, Vec<Expression>),
    BRewrite(Option<Expression>, Expression, Expression, Vec<Expression>),
    /// Formarly known as diff applier
    AddSearcher(Option<Expression>, Expression, Expression, Vec<Expression>),
}

pub type Parameter = Identifier;
pub type Constructor = (Identifier, Vec<Parameter>);

#[wasm_bindgen]
#[derive(Debug)]
pub enum Statement {
    /// Name and rewrite definition
    RewriteDef(String, Rewrite),
    /// Name - Params - Return type - Body
    Function(String, Vec<Parameter>, Annotation, Expression),
    /// Name - Type params - Constructors
    Datatype(String, Vec<Identifier>, Vec<Constructor>),
    /// Equality of two expressions with possible precondition
    Goal(Option<Expression>, Expression, Expression)
 }

pub type Identifier = String;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum Terminal {
    Id(Identifier, Option<Annotation>),
    Hole(Identifier, Option<Annotation>)
}

impl ToString for Terminal {
    fn to_string(&self) -> String {
        match self {
            Id(x, a) => x.to_string(),
            Hole(x, a) => x.to_string()
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Annotation {
    Type(Expression),
    Placeholder(usize),
    MultiAnnot(Vec<Annotation>),
}

impl Annotation {
    pub fn has_type(&self) -> bool {
        match self {
            Annotation::Type(_) => true,
            Annotation::Placeholder(_) => false,
            Annotation::MultiAnnot(x) => x.iter().any(|c| c.has_type())
        }
    }
}