use crate::logic_grammar::*;
use crate::ty_grammar::*;

fn not_in_stacks(env: Env, predicate: Predicate, prove_stacks: ProveStacks) -> bool {
    for pred in prove_stacks.0 .0.iter() {
        if predicate == *pred {
            return false;
        }
    }
    for pred in prove_stacks.1 .0.iter() {
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

pub fn reset(env_old: Env, var_ids: VarIds, env_new: Env) -> Env {
    let Env(hook_old, universe_old, _, _, _, hypotheses_old) = env_old;
    let Env(hook_new, _, var_binders_new, substitution_new, var_inequalities_new, _) = env_new;

    Env(
        hook_new,
        universe_old,
        var_binders_new,
        substitution_new,
        var_inequalities_new,
        hypotheses_old,
    )
}

pub fn substitution_to_fresh_vars<T: Subst>(term: T, kinded_var_ids: KindedVarIds) -> VarIdPairs {
    todo!()
}

pub fn env_with_incremented_universe(env: Env) -> Env {
    todo!()
}

pub fn env_with_vars_in_current_universe(
    env: Env,
    quantifier: Quantifier,
    kinded_var_ids: KindedVarIds,
) -> Env {
    todo!()
}
