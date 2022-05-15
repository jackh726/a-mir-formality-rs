use crate::instantiate;
use crate::logic_grammar::*;
use crate::ty_grammar::*;
use crate::utils;

pub fn prove(env: Env, prove_stacks: ProveStacks, goal: Goal) -> Option<Env> {
    match goal {
        Goal::AtomicGoal(AtomicGoal::Predicate(predicate)) => todo!(),
        Goal::AtomicGoal(AtomicGoal::Relation(relation)) => {
            let (env_eq, goals_eq) = utils::relate_parameters(env, relation);
            prove_all(env_eq, prove_stacks, goals_eq)
        }
        Goal::BuiltinGoal(BuiltinGoal::All(goals)) => prove_all(env, prove_stacks, *goals),
        Goal::BuiltinGoal(BuiltinGoal::Any(goals)) => {
            for goal in goals.0 {
                if let Some(env) = prove(env.clone(), prove_stacks.clone(), goal) {
                    return Some(env);
                }
            }
            None
        }
        Goal::BuiltinGoal(BuiltinGoal::Implies(hypotheses, goal)) => {
            let env_1 = utils::env_with_hypotheses(env.clone(), hypotheses);
            let env_out = prove(env_1, prove_stacks, *goal)?;
            Some(utils::reset(env, VarIds(vec![]), env_out))
        }
        Goal::BuiltinGoal(BuiltinGoal::Quantified(quantifier, kinded_var_ids, goal)) => {
            let (env_1, goal_1, varids_new) =
                instantiate::instantiate_qualified(env.clone(), quantifier, kinded_var_ids, *goal);
            let env_out = prove(env_1, prove_stacks, goal_1)?;
            Some(utils::reset(env, varids_new, env_out))
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
