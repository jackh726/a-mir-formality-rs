use crate::instantiate;
use crate::logic_grammar::*;
use crate::ty_grammar::*;
use crate::utils;

pub fn prove(env: Env, prove_stacks: ProveStacks, goal: Goal) -> Option<Env> {
    match goal {
        Goal::AtomicGoal(AtomicGoal::Predicate(predicate)) => todo!(),
        Goal::AtomicGoal(AtomicGoal::Relation(relation)) => todo!(),
        Goal::BuiltinGoal(BuiltinGoal::All(goals)) => prove_all(env, prove_stacks, *goals),
        Goal::BuiltinGoal(BuiltinGoal::Any(goals)) => todo!(),
        Goal::BuiltinGoal(BuiltinGoal::Implies(hypotheses, goal)) => todo!(),
        Goal::BuiltinGoal(BuiltinGoal::Quantified(quantifier, kinded_var_ids, goal)) => {
            let (env, goal, varids_new) =
                instantiate::instantiate_qualified(env, quantifier, kinded_var_ids, *goal);
            prove(env, prove_stacks, goal)
        }
    }
}

pub fn clause_proves_by_clause_fact(
    env: Env,
    prove_stacks: ProveStacks,
    prove_coinductive: ProveCoinductive,
    clause: Clause,
) -> Option<Env> {
    todo!()
}

pub fn prove_all(mut env: Env, mut prove_stacks: ProveStacks, mut goals: Goals) -> Option<Env> {
    let mut goals = goals.0;
    goals.reverse();

    while let Some(goal) = goals.pop() {
        env = prove(env, prove_stacks.clone(), goal)?;
        (goals, prove_stacks) =
            utils::apply_substitution_from_env(env.clone(), (goals, prove_stacks));
    }

    Some(env)
}
