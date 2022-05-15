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

pub fn env_with_hypotheses(env: Env, hypotheses: Hypotheses) -> Env {
    todo!()
}

pub fn env_contains_existential_var(env: Env, var_id: VarId) -> bool {
    for binder in env.2.0 {
        let bound_var_id = binder.0;
        let bound_quantifier = binder.2;
        if bound_var_id == var_id && matches!(bound_quantifier, Quantifier::Exists) {
            return true;
        }
    }
    return false;
}

pub fn relate_parameters(env: Env, relation: Relation) -> (Env, Goals) {
    let relation = apply_substitution_from_env(env.clone(), relation);
    match relation.1 {
        RelationOp::Equals => equate_one_substituted(env, relation.0, relation.2),
        RelationOp::InequalityOp(InequalityOp::SubtypeOp(subtype_op)) => {
            compare_one_substituted(env, relation.0, subtype_op, relation.2)
        }
        RelationOp::InequalityOp(InequalityOp::OutlivesOp(outlives_op)) => {
            outlives_one_substituted(env, relation.0, relation.2)
        }
    }
}

pub fn equate_one_substituted(
    env: Env,
    parameter_1: Parameter,
    parameter_2: Parameter,
) -> (Env, Goals) {
    if parameter_1 == parameter_2 {
        return (env, Goals(vec![]));
    }

    match (parameter_1, parameter_2) {
        (Parameter::Ty(ty_1), Parameter::Ty(ty_2)) => equate_one_substituted_tys(env, ty_1, ty_2),
        (Parameter::Lt(lt_1), Parameter::Lt(lt_2)) => equate_one_substituted_lts(env, lt_1, lt_2),
        _ => panic!(),
    }
}

fn equate_one_substituted_tys(env: Env, ty_1: Ty, ty_2: Ty) -> (Env, Goals) {
    todo!()
}

fn equate_one_substituted_lts(env: Env, lt_1: Lt, lt_2: Lt) -> (Env, Goals) {
    todo!()
}

pub fn compare_one_substituted(
    env: Env,
    parameter_1: Parameter,
    subtype_op: SubtypeOp,
    parameter_2: Parameter,
) -> (Env, Goals) {
    if parameter_1 == parameter_2 {
        return (env, Goals(vec![]));
    }

    match (parameter_1, parameter_2) {
        (Parameter::Ty(ty_1), Parameter::Ty(ty_2)) => {
            compare_one_substituted_tys(env, ty_1, subtype_op, ty_2)
        }
        (Parameter::Lt(lt_1), Parameter::Lt(lt_2)) => compare_one_substituted_lts(env, lt_1, lt_2),
        _ => panic!(),
    }
}

fn compare_one_substituted_tys(
    env: Env,
    ty_1: Ty,
    subtype_op: SubtypeOp,
    ty_2: Ty,
) -> (Env, Goals) {
    if ty_1 == ty_2 {
        return (env, Goals(vec![]));
    }

    match (ty_1, subtype_op, ty_2) {
        (Ty::VarId(var_id), _, Ty::RigidTy(RigidTy(rigid_name, parameters)))
            if env_contains_existential_var(env.clone(), var_id.clone()) =>
        {
            todo!()
        }
        (Ty::RigidTy(RigidTy(rigid_name, parameters)), _, Ty::VarId(var_id))
            if env_contains_existential_var(env.clone(), var_id.clone()) =>
        {
            todo!()
        }
        (
            Ty::RigidTy(RigidTy(rigid_name_1, parameters_1)),
            _,
            Ty::RigidTy(RigidTy(rigid_name_2, parameters_2)),
        ) if rigid_name_1 == rigid_name_2 => {
            relate_rigid_to_rigid(env, rigid_name_1, parameters_1, parameters_2)
        }
        (Ty::VarId(var_id), _, ty_2)
            if env_contains_existential_var(env.clone(), var_id.clone())
                && occurs_check_ok(env.clone(), var_id.clone(), ty_2.clone())
                && univserse_check_ok(env.clone(), var_id.clone(), ty_2.clone()) =>
        {
            todo!()
        }
        (ty_1, _, Ty::VarId(var_id))
            if env_contains_existential_var(env.clone(), var_id.clone())
                && occurs_check_ok(env.clone(), var_id.clone(), ty_1.clone())
                && univserse_check_ok(env.clone(), var_id.clone(), ty_1.clone()) =>
        {
            todo!()
        }
        (Ty::VarId(var_id), _, ty_2)
            if env_contains_existential_var(env.clone(), var_id.clone())
                && occurs_check_ok(env.clone(), var_id.clone(), ty_2.clone())
                && !univserse_check_ok(env.clone(), var_id.clone(), ty_2.clone()) =>
        {
            todo!()
        }
        (ty_1, _, Ty::VarId(var_id))
            if env_contains_existential_var(env.clone(), var_id.clone())
                && occurs_check_ok(env.clone(), var_id.clone(), ty_1.clone())
                && !univserse_check_ok(env.clone(), var_id.clone(), ty_1.clone()) =>
        {
            todo!()
        }
        (ty_1, SubtypeOp::Superset, ty_2) => {
            compare_one_substituted_tys(env, ty_2, SubtypeOp::Subset, ty_1)
        }
        (
            ty_1,
            SubtypeOp::Subset,
            Ty::PredicateTy(PredicateTy::ForAllTy(ForAllTy(kinded_var_ids, ty_2))),
        ) => {
            // FIXME: binding with free names
            (
                env,
                Goals(vec![Goal::BuiltinGoal(BuiltinGoal::Quantified(
                    Quantifier::ForAll,
                    kinded_var_ids,
                    Box::new(Goal::AtomicGoal(AtomicGoal::Relation(Relation(
                        Parameter::Ty(ty_1),
                        RelationOp::InequalityOp(InequalityOp::SubtypeOp(SubtypeOp::Subset)),
                        Parameter::Ty(*ty_2),
                    )))),
                ))]),
            )
        }
        (
            ty_1,
            SubtypeOp::Subset,
            Ty::PredicateTy(PredicateTy::ImplicationTy(ImplicationTy(where_clauses, ty_2))),
        ) => {
            // FIXME: should be where_clauses_to_goals
            let hypotheses = where_clauses_to_hypotheses(where_clauses);
            (
                env,
                Goals(vec![Goal::BuiltinGoal(BuiltinGoal::Implies(
                    hypotheses,
                    Box::new(Goal::AtomicGoal(AtomicGoal::Relation(Relation(
                        Parameter::Ty(ty_1),
                        RelationOp::InequalityOp(InequalityOp::SubtypeOp(SubtypeOp::Subset)),
                        Parameter::Ty(*ty_2),
                    )))),
                ))]),
            )
        }
        (
            Ty::PredicateTy(PredicateTy::EnsuresTy(EnsuresTy(ty_1, where_clauses))),
            SubtypeOp::Subset,
            ty_2,
        ) => {
            let hypotheses = where_clauses_to_hypotheses(where_clauses);
            (
                env,
                Goals(vec![Goal::BuiltinGoal(BuiltinGoal::Implies(
                    hypotheses,
                    Box::new(Goal::AtomicGoal(AtomicGoal::Relation(Relation(
                        Parameter::Ty(*ty_1),
                        RelationOp::InequalityOp(InequalityOp::SubtypeOp(SubtypeOp::Subset)),
                        Parameter::Ty(ty_2),
                    )))),
                ))]),
            )
        }
        (
            Ty::PredicateTy(PredicateTy::ForAllTy(ForAllTy(kinded_var_ids, ty_1))),
            SubtypeOp::Subset,
            ty_2,
        ) => {
            // FIXME: binding with free names
            (
                env,
                Goals(vec![Goal::BuiltinGoal(BuiltinGoal::Quantified(
                    Quantifier::Exists,
                    kinded_var_ids,
                    Box::new(Goal::AtomicGoal(AtomicGoal::Relation(Relation(
                        Parameter::Ty(*ty_1),
                        RelationOp::InequalityOp(InequalityOp::SubtypeOp(SubtypeOp::Subset)),
                        Parameter::Ty(ty_2),
                    )))),
                ))]),
            )
        }
        (
            Ty::PredicateTy(PredicateTy::ImplicationTy(ImplicationTy(where_clause, ty_1))),
            SubtypeOp::Subset,
            ty_2,
        ) => todo!(),
        (
            ty_1,
            SubtypeOp::Subset,
            Ty::PredicateTy(PredicateTy::EnsuresTy(EnsuresTy(ty_2, where_clause))),
        ) => todo!(),
        (
            Ty::AliasTy(AliasTy(alias_name_1, parameters_1)),
            SubtypeOp::Subset,
            Ty::AliasTy(AliasTy(alias_name_2, parameter_2)),
        ) if alias_name_1 == alias_name_2 => todo!(),
        (Ty::AliasTy(AliasTy(alias_name, parameters)), SubtypeOp::Subset, ty_2) => todo!(),
        (ty_1, SubtypeOp::Subset, Ty::AliasTy(AliasTy(alias_name, parameters))) => todo!(),
        _ => todo!(),
    }
}

fn compare_one_substituted_lts(env: Env, lt_1: Lt, lt_2: Lt) -> (Env, Goals) {
    todo!()
}

pub fn outlives_one_substituted(
    env: Env,
    parameter_1: Parameter,
    parameter_2: Parameter,
) -> (Env, Goals) {
    todo!()
}

pub fn relate_var_to_rigid(
    env: Env,
    var_id: VarId,
    relation_op: RelationOp,
    rigid_name: RigidName,
    parameters: Parameters,
) -> (Env, Goals) {
    todo!()
}

pub fn relate_rigid_to_rigid(
    env: Env,
    rigid_name: RigidName,
    parameters_1: Parameters,
    parameters_2: Parameters,
) -> (Env, Goals) {
    todo!()
}

pub fn occurs_check_ok(env: Env, var_id: VarId, ty: Ty) -> bool {
    todo!()
}

pub fn univserse_check_ok(env: Env, var_id: VarId, ty: Ty) -> bool {
    todo!()
}

pub fn where_clauses_to_goals(where_clauses: WhereClauses) -> Goals {
    todo!()
}

pub fn where_clauses_to_hypotheses(where_clauses: WhereClauses) -> Hypotheses {
    todo!()
}
