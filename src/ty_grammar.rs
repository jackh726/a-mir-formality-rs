#![allow(unused)]

pub struct Schemes(pub Vec<Schemes>);

pub struct Scheme;

#[derive(Clone, PartialEq, Eq)]
pub enum ParameterKind {
    Ty,
    Lt,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameters(pub Vec<Parameter>);
#[derive(Clone, PartialEq, Eq)]
pub enum Parameter {
    Ty(Ty),
    Lt(Lt),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Predicate {
    Implemented(TraitRef),
    HasImpl(TraitRef),
    WellFormed(ParameterKind, Parameter),
    Normalize(AliasTy, Ty),
}

pub struct PredicateDeboned(PredicateSkeleton, Parameters);
pub enum PredicateSkeleton {
    Implemented(TraitId),
    HasImpl(TraitId),
    WellFormed(ParameterKind),
    Normalize(AliasName),
}

#[derive(Clone, PartialEq, Eq)]
pub struct WhereClauses(pub Vec<WhereClause>);
#[derive(Clone, PartialEq, Eq)]
pub enum WhereClause {
    ForAll(KindedVarIds, Box<WhereClause>),
    Implemented(TraitRef),
    Outlives(Parameter, Lt),
    Normalize(AliasTy, Ty),
}

pub struct Tys(pub Vec<Ty>);
#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
    RigidTy(RigidTy),
    AliasTy(AliasTy),
    PredicateTy(PredicateTy),
    VarId(VarId),
}

#[derive(Clone, PartialEq, Eq)]
pub struct RigidTy(pub RigidName, pub Parameters);

#[derive(Clone, PartialEq, Eq)]
pub enum RigidName {
    AdtId(AdtId),
    ScalarId(ScalarId),
    Ref(MaybeMut),
    Tuple(usize),
    Fn(Abi, usize),
}

#[derive(Clone, PartialEq, Eq)]
pub struct AliasTy(pub AliasName, pub Parameters);
#[derive(Clone, PartialEq, Eq)]
pub enum AliasName {
    AliasId(AliasId),
    TraitId(AssociatedTyId),
}

#[derive(Clone, PartialEq, Eq)]
pub enum PredicateTy {
    ForAllTy(ForAllTy),
    ExistsTy(ExistsTy),
    ImplicationTy(ImplicationTy),
    EnsuresTy(EnsuresTy),
}
#[derive(Clone, PartialEq, Eq)]
pub struct ForAllTy(pub KindedVarIds, pub Box<Ty>);
#[derive(Clone, PartialEq, Eq)]
pub struct ImplicationTy(pub WhereClauses, pub Box<Ty>);
#[derive(Clone, PartialEq, Eq)]
pub struct ExistsTy(KindedVarIds, Box<Ty>);
#[derive(Clone, PartialEq, Eq)]
pub struct EnsuresTy(pub Box<Ty>, pub WhereClauses);

#[derive(Clone, PartialEq, Eq)]
pub struct Abi(pub String);

#[derive(Clone, PartialEq, Eq)]
pub enum Lt {
    Static,
    VarId(VarId),
}

#[derive(Clone, PartialEq, Eq)]
pub struct TraitRef(pub TraitId, pub Parameters);
pub struct AssociatedTy(pub TraitId, pub AssociatedTyId);

#[derive(Clone, PartialEq, Eq)]
pub enum MaybeMut {
    Not,
    Mut,
}

#[derive(Clone, PartialEq, Eq)]
pub struct VarIdPairs(pub Vec<VarIdPair>);
#[derive(Clone, PartialEq, Eq)]
pub struct VarIdPair(pub VarId, pub VarId);

#[derive(Clone, PartialEq, Eq)]
pub struct VarInequality(pub VarId, pub InequalityOp, pub Parameters);

#[derive(Clone, PartialEq, Eq)]
pub enum InequalityOp {
    SubtypeOp(SubtypeOp),
    OutlivesOp(OutlivesOp),
}

#[derive(Clone, PartialEq, Eq)]
pub enum SubtypeOp {
    Subset,
    Superset,
}

#[derive(Clone, PartialEq, Eq)]
pub enum OutlivesOp {
    Outlives,
    OutlivedBy,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ScalarId {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Bool,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AdtId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct AliasId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct TraitId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct AssociatedTyId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct TyAliasId(pub String);
#[derive(Clone, PartialEq, Eq)]
pub struct VarId(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct KindedVarIds(pub Vec<KindedVarId>);
#[derive(Clone, PartialEq, Eq)]
pub struct KindedVarId(pub Parameter, pub VarId);

pub struct Generics(GenericParameters, WhereClauses);
pub struct GenericParameters(pub Vec<GenericParameter>);
pub struct GenericParameter(VarId, KindAndVariance);
pub struct KindAndVariance(ParameterKind, Variance);
pub enum Variance {
    Less,
    More,
    Equal,
}

const TY_UNIT: RigidTy = RigidTy(RigidName::Tuple(0), Parameters(vec![]));

macro_rules! scalar_ty {
    ( $id:expr ) => {
        RigidTy($id, vec![])
    };
}

macro_rules! mk_ref {
    ( $lt:expr; $ty:expr ) => {
        RigidTy(RigidName::Ref(MaybeMut::Not), vec![$lt, $ty])
    };
}

macro_rules! mk_mut_ref {
    ( $lt:expr; $ty:expr ) => {
        RigidTy(RigidName::Ref(MaybeMut::Mut), vec![$lt, $ty])
    };
}
