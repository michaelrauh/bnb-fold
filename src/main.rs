use bnb_fold::solve::solve_for_dims;


include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn main() {
    // solve_for_dims(vec![4, 4]);
    dbg!(KEYWORDS.contains("loop"));
}
