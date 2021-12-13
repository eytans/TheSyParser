use std::collections::HashSet;
use crate::ast::{Constructor, Statement, Terminal};
use crate::Statement::Datatype;
use std::iter::Iterator;
use std::collections::HashMap;
use lalrpop_util::ParseError;

fn parse_datatype(name: String, type_params: Option<(String, Vec<String>)>, constrs: Vec<Constructor>) -> Option<Statement> {
    if let Some((h, t)) = type_params {
        let mut tps = vec![h];
        tps.extend(t);
        Some(Datatype(name, tps, constrs))
    } else {
        Some(Datatype(name, vec![], constrs))
    }
}


pub fn holes_correspond<'a, L , T>(terminals: impl Iterator<Item=&'a Terminal>) -> Result<(), ParseError<L, T, String>> {
    let mut holes = HashSet::new();
    let mut tems = HashSet::new();
    for t in terminals {
        let id = t.ident();
        if t.is_hole() {
            if tems.contains(id) {
                return Err(ParseError::User { error:
                    format!("Identifier {:?} is used both as a hole and as a normal id", id)
                });
            }
            holes.insert(id);
        } else {
            if holes.contains(id) {
                return Err(ParseError::User { error:
                    format!("Identifier {:?} is used both as a hole and as a normal id", id)
                });
            }
            tems.insert(id);
        }
    }
    return Ok(());
}