use crate::ast::Terminal::{Id, Hole};
use itertools::Itertools;
use std::fmt::Formatter;
use crate::ast::Rewrite::{DRewrite, BRewrite, AddSearcher};

#[derive(Debug, Clone)]
pub enum Definitions {
    Defs(Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Leaf(Terminal),
    Op(Terminal, Vec<Expression>),
    Match(Box<Expression>, Vec<(Expression, Expression)>)
}

lazy_static! {
  static ref MATCH_ID: Terminal = Id(String::from("match"), None);
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
            Expression::Match(root, patterns) => {
                let mut res: Vec<&Terminal> = patterns.iter().flat_map(|(c, b)| {
                    let mut res = c.terminals();
                    res.extend(b.terminals());
                    res
                }).collect_vec();
                res.extend(root.terminals());
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
            Expression::Match(root, pats) => {
                format!("(match {} {})", root.to_sexp_string(), pats.iter()
                    .map(|(constr, body)|
                        format!("(=> {} {})", constr.to_sexp_string(), body.to_sexp_string()))
                    .join(" "))
            }
        }
    }

    pub fn root(&self) -> &Terminal {
        match self {
            Expression::Leaf(x) => {x}
            Expression::Op(x, _) => {x}
            Expression::Match(_, _) => {&MATCH_ID}
        }
    }

    pub fn children(&self) -> Vec<Expression> {
        match self {
            Expression::Leaf(_) => {vec![]}
            Expression::Op(_, cs) => {cs.clone()}
            Expression::Match(root, pats) => {
                let mut res = pats.iter()
                    .map(|x| x.1.clone()).collect_vec();
                res.push(*root.clone());
                res
            }
        }
    }

    pub fn map(&self, f: &mut impl FnMut(&Terminal) -> Terminal) -> Self {
        match self {
            Expression::Leaf(t) => {
                Expression::Leaf(f(t))
            }
            Expression::Op(t, children) => {
                Expression::Op(f(t), children.iter().map(|c| c.map(f)).collect_vec())
            }
            Expression::Match(root, pats) => {
                Expression::Match(
                    Box::new(root.map(f)),
                    pats.iter()
                        .map(|(ctr, pat)| (ctr.map(f), pat.map(f)))
                        .collect_vec()
                )
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_sexp_string())
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
    DRewrite(Option<Expression>, Expression, Expression, Vec<Condition>),
    BRewrite(Option<Expression>, Expression, Expression, Vec<Condition>),
    /// Formarly known as diff applier
    AddSearcher(Option<Expression>, Expression, Expression, Vec<Condition>),
}

impl Rewrite {
    pub fn source_expressions(&self) -> Vec<&Expression> {
        match self {
            DRewrite(_, source, _, _) => {vec![source]}
            BRewrite(_, source, target, _) => {vec![source, target]}
            AddSearcher(_, source, _, _) => {vec![source]}
        }
    }
}

pub type Parameter = (Identifier, Annotation);
pub type Constructor = (Identifier, Vec<Parameter>);
pub type Condition = (Expression, Expression);

#[derive(Debug, Clone)]
pub enum Statement {
    /// Name and rewrite definition
    RewriteDef(String, Rewrite),
    /// Name - Params - Return type - Body
    Function(String, Vec<Parameter>, Annotation, Option<Expression>),
    /// Name - Type params - Constructors
    Datatype(String, Vec<Identifier>, Vec<Constructor>),
    /// Equality of two expressions with possible precondition
    Goal(Option<Expression>, Expression, Expression),
    /// Case split: searcher - sub expression - patterns subexpression turns to - conditions for searcher
    CaseSplit(Expression, Expression, Vec<Expression>, Vec<Condition>)
 }

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn anno(&self) -> &Option<Box<Annotation>> {
        match self {
            Id(_, a) => {a}
            Hole(_, a) => {a}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn get_ph(&self) -> Option<usize> {
        match self {
            Annotation::Type(_) => None,
            Annotation::Placeholder(x) => Some(*x),
            Annotation::MultiAnnot(x) => x.iter().find_map(|c| c.get_ph())
        }
    }
}