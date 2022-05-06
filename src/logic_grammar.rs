#![allow(unused)]

pub struct Hook;

pub struct Parameter(Term);
pub struct ParameterKind(Term);
pub struct Predicate(Term);
pub struct VarInequality(Term);
pub struct InequalityOp(Term);

pub struct Envs(Vec<Env>);
pub struct Env(Hook, Universe, VarBinders, Substitution, VarInequalities, Hypotheses);

pub struct Substitution(Vec<(VarId, Parameter)>);
pub enum SubstitutionOrError {
    Substitution(Substitution),
    Error(Error),
}

pub struct VarBinders(Vec<VarBinder>);
pub struct VarBinder(VarId, ParameterKind, Quantifier, Universe);

pub struct VarInequalities(Vec<VarInequality>);

pub struct KindedVarIds(Vec<KindedVarId>);
pub struct KindedVarId(Parameter, VarId);

pub struct Parameters(Vec<Parameter>);

pub struct Predicates(Vec<Predicate>);

pub struct Goals(Vec<Goal>);
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

pub struct Hypotheses(Vec<Hypothesis>);
pub struct Clauses(Vec<Clause>);
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

pub struct FlatHypotheses(Vec<FlatHypothesis>);
pub struct FlatHypothesis(KindedVarIds, FlatImplicationHypothesis); // ForAll
pub struct FlatImplicationHypotheses(Vec<FlatImplicationHypothesis>);
pub struct FlatImplicationHypothesis(Goals, AtomicGoal); // Implies

pub struct Invariants(Vec<Invariant>);
pub struct Invariant(KindedVarIds, Predicate, Predicate); // ForAll KindedVarIds (Implies (Predicate) Predicate))

pub struct Relations(Vec<Relation>);
pub struct Relation(Parameter, RelationOp, Parameter);
pub enum RelationOp {
    Equals,
    InequalityOp(InequalityOp),
}

pub enum Quantifier {
    ForAll,
    Exists,
}

pub struct Universe(UniverseId, usize);

pub struct VarIds(Vec<VarId>);
pub struct VarId(String);
pub struct AnyId(String);
pub struct UniverseId(String);

pub struct Error(String);

pub struct Terms(Vec<Term>);
pub struct Term(String);
pub struct TermPair(Term, Term);
pub struct TermPairs(Vec<TermPair>);

pub struct ProveStacks(Predicates, Predicates);
pub enum ProveCoinductive {
    More,
    Less,
}

//pub const ROOT_UNIVERSE: Universe = Universe(UniverseId(String::from("root")), 0);