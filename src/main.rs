pub mod rule;
mod solve;
pub mod string_handlers;

fn main() {
    solve::solve_for_dims(vec![2, 2])
}
