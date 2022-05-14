use crate::ty_grammar::*;
use crate::logic_grammar::*;

// prove-exists
fn not_in_stacks(env: Env, predicate: Predicate, prove_stacks: ProveStacks) -> bool {
    for pred in prove_stacks.0.0.iter() {
        if predicate == *pred {
            return false;
        }
    }
    for pred in prove_stacks.1.0.iter() {
        if predicate == *pred {
            return false;
        }
    }
    return true;
}