use crate::logic_grammar::{self, *};
use crate::ty_grammar::{self, *};

fn test_subtyping() {
    let root_univsere = Universe(UniverseId(String::from("root")), 0);
    let env: Env = Env(
        Hook,
        root_univsere,
        VarBinders(vec![]),
        Substitution(vec![]),
        VarInequalities(vec![]),
        Hypotheses(vec![
            Hypothesis::ForAll(
                logic_grammar::KindedVarIds(vec![]),
                Box::new(Hypothesis::AtomicGoal(AtomicGoal::Predicate(
                    logic_grammar::Predicate(ty_grammar::Predicate::WellFormed(
                        ty_grammar::ParameterKind::Ty,
                        ty_grammar::Parameter::Ty(Ty::VarId(ty_grammar::VarId("T".into()))),
                    )),
                ))),
            ),
            Hypothesis::ForAll(
                logic_grammar::KindedVarIds(vec![]),
                Box::new(Hypothesis::AtomicGoal(AtomicGoal::Predicate(
                    logic_grammar::Predicate(ty_grammar::Predicate::Implemented(TraitRef(
                        TraitId("AlwwaysImpl".into()),
                        ty_grammar::Parameters(vec![ty_grammar::Parameter::Ty(Ty::VarId(
                            ty_grammar::VarId("T".into()),
                        ))]),
                    ))),
                ))),
            ),
        ]),
    );
}
