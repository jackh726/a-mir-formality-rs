#![allow(unused)]

use crate::ty_grammar::*;

pub struct Hook;

pub struct Envs(pub Vec<Env>);
pub struct Env(
    pub Hook,
    pub Universe,
    pub VarBinders,
    pub Substitution,
    pub VarInequalities,
    pub Hypotheses,
);

pub struct Substitution(pub Vec<(VarId, Parameter)>);
pub enum SubstitutionOrError {
    Substitution(Substitution),
    Error(Error),
}

pub struct VarBinders(pub Vec<VarBinder>);
pub struct VarBinder(VarId, ParameterKind, Quantifier, Universe);

pub struct VarInequalities(pub Vec<VarInequality>);

pub struct KindedVarIds(pub Vec<KindedVarId>);
pub struct KindedVarId(Parameter, VarId);

pub struct Parameters(pub Vec<Parameter>);

pub struct Predicates(pub Vec<Predicate>);

pub struct Goals(pub Vec<Goal>);
pub enum Goal {
    AtomicGoal(AtomicGoal),
    BuiltinGoal(BuiltinGoal),
}
pub enum AtomicGoal {
    Predicate(Predicate),
    Relation(Relation),
}
pub enum BuiltinGoal {
    All(Box<Goals>),
    Any(Box<Goals>),
    Implies(Hypotheses, Box<Goal>),
    // FIXME: change in a-mir-formality
    Quantified(Quantifier, KindedVarIds, Box<Goal>),
}

pub struct Hypotheses(pub Vec<Hypothesis>);
pub struct Clauses(pub Vec<Clause>);
pub enum Hypothesis {
    AtomicGoal(AtomicGoal),
    Implies(Goals, AtomicGoal),
    ForAll(KindedVarIds, Box<Hypothesis>),
}
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
pub struct Relation(Parameter, RelationOp, Parameter);
pub enum RelationOp {
    Equals,
    InequalityOp(InequalityOp),
}

pub enum Quantifier {
    ForAll,
    Exists,
}

pub struct Universe(pub UniverseId, pub usize);

pub struct VarIds(pub Vec<VarId>);
pub struct VarId(pub String);
pub struct AnyId(pub String);
pub struct UniverseId(pub String);

pub struct Error(pub String);

pub struct Terms(pub Vec<Term>);
pub struct Term(pub String);
pub struct TermPair(Term, Term);
pub struct TermPairs(pub Vec<TermPair>);

pub struct ProveStacks(Predicates, Predicates);
pub enum ProveCoinductive {
    More,
    Less,
}

//pub const ROOT_UNIVERSE: Universe = Universe(UniverseId(pub String::from("root")), 0);
