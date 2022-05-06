#![allow(unused)]

pub struct Schemes(Vec<Schemes>);

pub struct Scheme;

pub enum ParameterKind {
    TyKind(TyKind),
    LtKind(LtKind),
}

pub struct TyKind;
pub struct LtKind;

pub struct Parameters(Vec<Parameter>);
pub enum Parameter {
    Ty(Ty),
    Lt(Lt),
}

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

pub struct WhereClauses(Vec<WhereClause>);
pub enum WhereClause {
    ForAll(KindedVarIds, Box<WhereClause>),
    Implemented(TraitRef),
    Outlives(Parameter, Lt),
    Normalize(AliasTy, Ty),
}

pub struct Tys(Vec<Ty>);
pub enum Ty {
    RigidTy(RigidTy),
    AliasTy(AliasTy),
    PredicateTy(PredicateTy),
    VaId(VarId),
}

pub struct RigidTy(RigidName, Parameters);

pub enum RigidName {
    AdtId(AdtId),
    ScalarId(ScalarId),
    Ref(MaybeMut),
    Tuple(usize),
    Fn(Abi, usize),
}

pub struct AliasTy(AliasName, Parameters);
pub enum AliasName {
    AliasId(AliasId),
    TraitId(AssociatedTyId),
}

pub enum PredicateTy {
    ForAllTy(ForAllTy),
    ExistsTy(ExistsTy),
    ImplicationTy(ImplicationTy),
    EnsuresTy(EnsuresTy),
}
pub struct ForAllTy(KindedVarIds, Box<Ty>);
pub struct ImplicationTy(WhereClauses, Box<Ty>);
pub struct ExistsTy(KindedVarIds, Box<Ty>);
pub struct EnsuresTy(Box<Ty>, WhereClauses);

pub struct Abi(String);

pub enum Lt {
    Static,
    VarId(VarId),
}

pub struct TraitRef(TraitId, Parameters);
pub struct AssociatedTy(TraitId, AssociatedTyId);

pub enum MaybeMut {
    Not,
    Mut,
}

pub struct VarIdPaids(Vec<VarIdPair>);
pub struct VarIdPair(VarId, VarId);

pub struct VarInequality(VarId, InequalityOp, Parameters);

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

pub struct AdtId(String);
pub struct AliasId(String);
pub struct TraitId(String);
pub struct AssociatedTyId(String);
pub struct TyAliasId(String);
pub struct VarId(String);

pub struct KindedVarIds;

pub struct Generics(GenericParameters, WhereClauses);
pub struct GenericParameters(Vec<GenericParameter>);
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
