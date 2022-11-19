extern crate lalrpop;

fn main() {
    let res = lalrpop::process_root();
    res.unwrap_or_else(|x| {
        println!("{}", x);
        panic!("{}", x)
    });
}