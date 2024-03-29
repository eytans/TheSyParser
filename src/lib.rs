pub mod ast;
mod parse_tools;

#[macro_use] extern crate log;
#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
lalrpop_mod!(pub grammar);

use crate::ast::Statement;


#[cfg(test)]
mod tests {
    use env_logger;
    use crate::grammar;

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

    #[test]
    fn rw_defs() {
        let defs = grammar::DefsParser::new().parse("rw test a => b\n\n");
        match defs {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{:?}", x);
                assert!(false, "Error parsing Defs");
            }
        };
    }

    #[test]
    fn datatype_stmt() {
        let dt = grammar::StmtParser::new().parse("datatype Lst () := (nil : (ret : Lst))");
        match dt {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{:?}", x);
                assert!(false, "Error parsing rewrite");
            }
        };
    }

    #[test]
    fn fun_no_body_stmt() {
        let f = grammar::StmtParser::new().parse("fun append l1: Lst l2: Lst -> Lst");
        match f {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{:?}", x);
                assert!(false, "Error parsing rewrite");
            }
        };
    }

    #[test]
    fn multiline_defs() {
        let defs = grammar::DefsParser::new().parse("datatype Lst () := (nil : (ret : Lst))
rw test a => b

fun append l1: Lst l2: Lst -> Lst");
        match defs {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{:?}", x);
                assert!(false, "Error parsing Defs");
            }
        };
    }

    #[test]
    fn expression_rw_with_hole() {
        let text = "rw app_base (append nil ?x) => ?x";
        let exp = "(append nil ?x)";
        let parser = grammar::ExpParser::new();
        let e = parser.parse(exp);
        match e {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{}", x);
                assert!(false, "Error parsing exp");
            }
        };
        let f = grammar::StmtParser::new().parse(text);
        match f {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{}", x);
                assert!(false, "Error parsing rewrite");
            }
        };
    }

    #[test]
    fn hole_or_id() {
        let text = "rw app_base (append nil ?x) => x";
        let f = grammar::StmtParser::new().parse(text);
        match f {
            Ok(s) => assert!(false, "Should fail as x is either a hole or an id can not be both"),
            Err(x) => {
                println!("{}", x);
            }
        };
    }

    #[test]
    fn parse_from_clamgoal1() {
        let text = "datatype Nat () := (succ : (x_0 : Nat) -> (res : Nat)) (zero : (res : Nat))
fun plus (x_0 : Nat) (x_1 : Nat) -> Nat
fun double (x_0 : Nat) -> Nat
fun leq (__x0 : Nat) (__y1 : Nat) -> Bool => (or (= ?__x0 ?__y1) (less ?__x0 ?__y1))";
        let defs = grammar::DefsParser::new().parse(text);
        match defs {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{}", x);
                assert!(false, "Error parsing Defs");
            }
        };
    }

    #[test]
    fn parse_from_leon_queue7() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        debug!("Starting test");
        let text = "rw rule29 (and (= ?x17 nil) (= ?y18 nil)) => (isEmpty (queue ?x17 ?y18))";
        let defs = grammar::DefsParser::new().parse(text);
        match defs {
            Ok(s) => println!("{:?}", s),
            Err(x) => {
                println!("{}", x);
                assert!(false, "Error parsing Defs");
            }
        };
    }
}