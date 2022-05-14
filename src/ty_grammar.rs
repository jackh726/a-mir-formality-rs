#![allow(unused)]

pub struct Schemes(pub Vec<Schemes>);

pub struct Scheme;

#[derive(PartialEq, Eq)]
pub enum ParameterKind {
    Ty,
    Lt,
}

#[derive(PartialEq, Eq)]
pub struct Parameters(pub Vec<Parameter>);
#[derive(PartialEq, Eq)]
pub enum Parameter {
    Ty(Ty),
    Lt(Lt),
}

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub struct WhereClauses(pub Vec<WhereClause>);
#[derive(PartialEq, Eq)]
pub enum WhereClause {
    ForAll(KindedVarIds, Box<WhereClause>),
    Implemented(TraitRef),
    Outlives(Parameter, Lt),
    Normalize(AliasTy, Ty),
}

pub struct Tys(pub Vec<Ty>);
#[derive(PartialEq, Eq)]
pub enum Ty {
    RigidTy(RigidTy),
    AliasTy(AliasTy),
    PredicateTy(PredicateTy),
    VarId(VarId),
}

#[derive(PartialEq, Eq)]
pub struct RigidTy(pub RigidName, pub Parameters);

#[derive(PartialEq, Eq)]
pub enum RigidName {
    AdtId(AdtId),
    ScalarId(ScalarId),
    Ref(MaybeMut),
    Tuple(usize),
    Fn(Abi, usize),
}

#[derive(PartialEq, Eq)]
pub struct AliasTy(AliasName, Parameters);
#[derive(PartialEq, Eq)]
pub enum AliasName {
    AliasId(AliasId),
    TraitId(AssociatedTyId),
}

#[derive(PartialEq, Eq)]
pub enum PredicateTy {
    ForAllTy(ForAllTy),
    ExistsTy(ExistsTy),
    ImplicationTy(ImplicationTy),
    EnsuresTy(EnsuresTy),
}
#[derive(PartialEq, Eq)]
pub struct ForAllTy(pub KindedVarIds, pub Box<Ty>);
#[derive(PartialEq, Eq)]
pub struct ImplicationTy(WhereClauses, Box<Ty>);
#[derive(PartialEq, Eq)]
pub struct ExistsTy(KindedVarIds, Box<Ty>);
#[derive(PartialEq, Eq)]
pub struct EnsuresTy(Box<Ty>, WhereClauses);

#[derive(PartialEq, Eq)]
pub struct Abi(pub String);

#[derive(PartialEq, Eq)]
pub enum Lt {
    Static,
    VarId(VarId),
}

#[derive(PartialEq, Eq)]
pub struct TraitRef(pub TraitId, pub Parameters);
pub struct AssociatedTy(pub TraitId, pub AssociatedTyId);

#[derive(PartialEq, Eq)]
pub enum MaybeMut {
    Not,
    Mut,
}

pub struct VarIdPaids(pub Vec<VarIdPair>);
pub struct VarIdPair(pub VarId, pub VarId);

pub struct VarInequality(pub VarId, pub InequalityOp, pub Parameters);

pub enum InequalityOp {
    SubtypeOp(SubtypeOp),
    OutlivesOp(OutlivesOp),
}

pub enum SubtypeOp {
    Subset,
    Superset,
}

pub enum OutlivesOp {
    Outlives,
    OutlivedBy,
}

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub struct AdtId(pub String);
#[derive(PartialEq, Eq)]
pub struct AliasId(pub String);
#[derive(PartialEq, Eq)]
pub struct TraitId(pub String);
#[derive(PartialEq, Eq)]
pub struct AssociatedTyId(pub String);
#[derive(PartialEq, Eq)]
pub struct TyAliasId(pub String);
#[derive(PartialEq, Eq)]
pub struct VarId(pub String);

#[derive(PartialEq, Eq)]
pub struct KindedVarIds(pub Vec<KindedVarId>);
#[derive(PartialEq, Eq)]
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
