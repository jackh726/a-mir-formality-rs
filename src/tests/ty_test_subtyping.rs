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
                ty_grammar::KindedVarIds(vec![KindedVarId(
                    Parameter::Ty(Ty::VarId(VarId("T".into()))),
                    VarId("T".into()),
                )]),
                Box::new(Hypothesis::AtomicGoal(AtomicGoal::Predicate(
                    ty_grammar::Predicate::WellFormed(
                        ty_grammar::ParameterKind::Ty,
                        ty_grammar::Parameter::Ty(Ty::VarId(ty_grammar::VarId("T".into()))),
                    ),
                ))),
            ),
            Hypothesis::ForAll(
                ty_grammar::KindedVarIds(vec![KindedVarId(
                    Parameter::Ty(Ty::VarId(VarId("T".into()))),
                    VarId("T".into()),
                )]),
                Box::new(Hypothesis::AtomicGoal(AtomicGoal::Predicate(
                    ty_grammar::Predicate::Implemented(TraitRef(
                        TraitId("AlwwaysImpl".into()),
                        ty_grammar::Parameters(vec![ty_grammar::Parameter::Ty(Ty::VarId(
                            ty_grammar::VarId("T".into()),
                        ))]),
                    )),
                ))),
            ),
        ]),
    );

    let goal: Goal = Goal::AtomicGoal(AtomicGoal::Relation(
        Relation(
            Parameter::Ty(Ty::PredicateTy(PredicateTy::ForAllTy(ForAllTy(ty_grammar::KindedVarIds(vec![ty_grammar::KindedVarId(
                ty_grammar::Parameter::Ty(Ty::VarId(VarId("T".into()))),
                VarId("T".into()),
            )]), Box::new(Ty::VarId(VarId("T".into()))))))),
            RelationOp::InequalityOp(InequalityOp::SubtypeOp(SubtypeOp::Subset)),
            Parameter::Ty(Ty::RigidTy(RigidTy(RigidName::ScalarId(ScalarId::U32), ty_grammar::Parameters(vec![])))),
        ),
    ));
}
