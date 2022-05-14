#![allow(unused)]

use crate::ty_grammar::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Hook;

pub struct Envs(pub Vec<Env>);
#[derive(Clone, PartialEq, Eq)]
pub struct Env(
    pub Hook,
    pub Universe,
    pub VarBinders,
    pub Substitution,
    pub VarInequalities,
    pub Hypotheses,
);

#[derive(Clone, PartialEq, Eq)]
pub struct Substitution(pub Vec<(VarId, Parameter)>);
pub enum SubstitutionOrError {
    Substitution(Substitution),
    Error(Error),
}

#[derive(Clone, PartialEq, Eq)]
pub struct VarBinders(pub Vec<VarBinder>);
#[derive(Clone, PartialEq, Eq)]
pub struct VarBinder(VarId, ParameterKind, Quantifier, Universe);

#[derive(Clone, PartialEq, Eq)]
pub struct VarInequalities(pub Vec<VarInequality>);

pub struct Parameters(pub Vec<Parameter>);

#[derive(Clone, PartialEq, Eq)]
pub struct Predicates(pub Vec<Predicate>);

#[derive(Clone, PartialEq, Eq)]
pub struct Goals(pub Vec<Goal>);
#[derive(Clone, PartialEq, Eq)]
pub enum Goal {
    AtomicGoal(AtomicGoal),
    BuiltinGoal(BuiltinGoal),
}
#[derive(Clone, PartialEq, Eq)]
pub enum AtomicGoal {
    Predicate(Predicate),
    Relation(Relation),
}
#[derive(Clone, PartialEq, Eq)]
pub enum BuiltinGoal {
    All(Box<Goals>),
    Any(Box<Goals>),
    Implies(Hypotheses, Box<Goal>),
    // FIXME: change in a-mir-formality
    Quantified(Quantifier, KindedVarIds, Box<Goal>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hypotheses(pub Vec<Hypothesis>);
#[derive(Clone, PartialEq, Eq)]
pub struct Clauses(pub Vec<Clause>);
#[derive(Clone, PartialEq, Eq)]
pub enum Hypothesis {
    AtomicGoal(AtomicGoal),
    Implies(Goals, AtomicGoal),
    ForAll(KindedVarIds, Box<Hypothesis>),
}
#[derive(Clone, PartialEq, Eq)]
pub enum Clause {
    AtomicGoal(AtomicGoal),
    Implies(Goals, AtomicGoal),
    ForAll(KindedVarIds, Box<Clause>),
}

pub struct FlatHypotheses(pub Vec<FlatHypothesis>);
pub struct FlatHypothesis(KindedVarIds, FlatImplicationHypothesis); // ForAll
pub struct FlatImplicationHypotheses(pub Vec<FlatImplicationHypothesis>);
pub struct FlatImplicationHypothesis(Goals, AtomicGoal); // Implies

pub struct Invariants(pub Vec<Invariant>);
pub struct Invariant(KindedVarIds, Predicate, Predicate); // ForAll KindedVarIds (Implies (Predicate) Predicate))

pub struct Relations(pub Vec<Relation>);
#[derive(Clone, PartialEq, Eq)]
pub struct Relation(pub Parameter, pub RelationOp, pub Parameter);
#[derive(Clone, PartialEq, Eq)]
pub enum RelationOp {
    Equals,
    InequalityOp(InequalityOp),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Quantifier {
    ForAll,
    Exists,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Universe(pub UniverseId, pub usize);

pub struct VarIds(pub Vec<VarId>);
pub struct AnyId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct UniverseId(pub String);

pub struct Error(pub String);

pub struct Terms(pub Vec<Term>);
pub struct Term(pub String);
pub struct TermPair(Term, Term);
pub struct TermPairs(pub Vec<TermPair>);

#[derive(Clone, PartialEq, Eq)]
pub struct ProveStacks(pub  Predicates, pub Predicates);
pub enum ProveCoinductive {
    More,
    Less,
}

//pub const ROOT_UNIVERSE: Universe = Universe(UniverseId(pub String::from("root")), 0);

pub trait Subst: Sized {
    fn subst(self, subst: Substitution) -> Self;
}

impl Subst for Ty {
    fn subst(self, subst: Substitution) -> Self {
        match self {
            Self::VarId(var_id) => {
                let mut new_ty = None;
                for (subst_var, param) in subst.0 {
                    if subst_var != subst_var {
                        continue;
                    }
                    match param {
                        Parameter::Ty(ty) => new_ty = Some(ty),
                        _ => panic!(),
                    }
                }
                match new_ty {
                    Some(new_ty) => new_ty,
                    None => Self::VarId(var_id),
                }
            }
            _ => todo!(),
        }
    }
}

impl Subst for Goal {
    fn subst(self, subst: Substitution) -> Self {
        todo!()
    }
}

impl Subst for ProveStacks {
    fn subst(self, subst: Substitution) -> Self {
        todo!()
    }
}

impl Subst for (Vec<Goal>, ProveStacks) {
    fn subst(self, subst: Substitution) -> Self {
        (
            self.0.into_iter().map(|g| g.subst(subst.clone())).collect(),
            self.1.subst(subst),
        )
    }
}
