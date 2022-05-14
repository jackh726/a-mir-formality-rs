use crate::ty_grammar::*;
use crate::logic_grammar::*;
use crate::utils;

pub fn instantiate_qualified<T: Clone + Subst>(env_0: Env, quantifier: Quantifier, kinded_var_ids: KindedVarIds, term_0: T) -> (Env, T, VarIds) {
    match quantifier {
        Quantifier::ForAll => {
            let subst_to_placeholders = utils::substitution_to_fresh_vars((env_0.clone(), term_0.clone()), kinded_var_ids.clone());
            let varids_new = VarIds(subst_to_placeholders.0.clone().into_iter().map(|s| s.1).collect());

            let env_1 = utils::env_with_incremented_universe(env_0);
            let vars = KindedVarIds(kinded_var_ids.0.into_iter().zip(varids_new.0.clone().into_iter()).map(|(kv, var_ids)| KindedVarId(kv.0, var_ids)).collect());
            let env_2 = utils::env_with_vars_in_current_universe(env_1, Quantifier::ForAll, vars);

            let term_1 = utils::apply_substitution(Substitution(subst_to_placeholders.0.into_iter().map(|pair| (pair.0, Parameter::Ty(Ty::VarId(pair.1)))).collect()), term_0);

            (env_2, term_1, varids_new)
        }
        Quantifier::Exists => {
            let subst_to_inference = utils::substitution_to_fresh_vars((env_0.clone(), term_0.clone()), kinded_var_ids.clone());
            let varids_new = VarIds(subst_to_inference.0.clone().into_iter().map(|s| s.1).collect());

            let vars = KindedVarIds(kinded_var_ids.0.into_iter().zip(varids_new.0.clone().into_iter()).map(|(kv, var_ids)| KindedVarId(kv.0, var_ids)).collect());
            let env_1 = utils::env_with_vars_in_current_universe(env_0, Quantifier::Exists, vars);

            let term_1 = utils::apply_substitution(Substitution(subst_to_inference.0.into_iter().map(|pair| (pair.0, Parameter::Ty(Ty::VarId(pair.1)))).collect()), term_0);

            (env_1, term_1, varids_new)
        }
    }
}
