pub mod rule;
pub mod solve;
pub mod string_handlers;
pub mod phrase_set;

fn main() {
    solve::solve_for_dims(vec![3, 3])
}
