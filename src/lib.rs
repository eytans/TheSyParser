pub mod ast;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use crate::ast::Statement;


#[test]
fn rw_statements() {
    let rw = grammar::StmtParser::new().parse("rw test a => b");
    match rw {
        Ok(s) => println!("{:?}", s),
        Err(x) => {
            println!("{:?}", x);
            assert!(false, "Error parsing rewrite");
        }
    };
}