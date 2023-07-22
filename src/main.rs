// use bnb_fold::solve::solve_for_dims;


include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn main() {
    // solve_for_dims(vec![4, 4]);

    dbg!(PHRASES.contains(&500));
    dbg!(VOCAB.contains(&1));
    dbg!(&DECODER.keys());
    dbg!(&VOCAB.iter());
}
