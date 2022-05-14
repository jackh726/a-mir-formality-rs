use crate::ty_grammar::*;
use crate::logic_grammar::*;

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

pub fn apply_substitution_from_env<T: Subst>(env: Env, term: T) -> T {
    let substitution = env.3;
    apply_substitution(substitution, term)
}

pub fn apply_substitution<T: Subst>(substitution: Substitution, term: T) -> T {
    // Case 1
    if substitution.0.is_empty() {
        return term;
    }
    // Case 2
    term.subst(substitution)
}
