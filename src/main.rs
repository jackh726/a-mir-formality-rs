mod decl_grammar;
mod logic_grammar;
mod ty_grammar;

mod cosld_solve;
mod instantiate;
mod utils;

mod tests;

fn main() {
    tests::ty_test_subtyping::test_subtyping();
}
