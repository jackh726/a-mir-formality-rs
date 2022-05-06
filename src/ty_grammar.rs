#![allow(unused)]

struct Schemes(Vec<Schemes>);

struct Scheme;

enum ParameterKind {
    TyKind(TyKind),
    LtKind(LtKind),
}

struct TyKind;
struct LtKind;

struct Parameters(Vec<Parameter>);
enum Parameter {
    Ty(Ty),
    Lt(Lt),
}

enum Predicate {
    Implemented(TraitRef),
    HasImpl(TraitRef),
    WellFormed(ParameterKind, Parameter),
    Normalize(AliasTy, Ty),
}

struct PredicateDeboned(PredicateSkeleton, Parameters);
enum PredicateSkeleton {
    Implemented(TraitId),
    HasImpl(TraitId),
    WellFormed(ParameterKind),
    Normalize(AliasName),
}

struct WhereClauses(Vec<WhereClause>);
enum WhereClause {
    ForAll(KindedVarIds, Box<WhereClause>),
    Implemented(TraitRef),
    Outlives(Parameter, Lt),
    Normalize(AliasTy, Ty),
}

struct Tys(Vec<Ty>);
enum Ty {
    RigidTy(RigidTy),
    AliasTy(AliasTy),
    PredicateTy(PredicateTy),
    VaId(VarId),
}

struct RigidTy(RigidName, Parameters);

enum RigidName {
    AdtId(AdtId),
    ScalarId(ScalarId),
    Ref(MaybeMut),
    Tuple(usize),
    Fn(Abi, usize),
}

struct AliasTy(AliasName, Parameters);
enum AliasName {
    AliasId(AliasId),
    TraitId(AssociatedTyId),
}

enum PredicateTy {
    ForAllTy(ForAllTy),
    ExistsTy(ExistsTy),
    ImplicationTy(ImplicationTy),
    EnsuresTy(EnsuresTy),
}
struct ForAllTy(KindedVarIds, Box<Ty>);
struct ImplicationTy(WhereClauses, Box<Ty>);
struct ExistsTy(KindedVarIds, Box<Ty>);
struct EnsuresTy(Box<Ty>, WhereClauses);

struct Abi(String);

enum Lt {
    Static,
    VarId(VarId),
}

struct TraitRef(TraitId, Parameters);
struct AssociatedTy(TraitId, AssociatedTyId);

enum MaybeMut {
    Not,
    Mut,
}

struct VarIdPaids(Vec<VarIdPair>);
struct VarIdPair(VarId, VarId);

struct VarInequality(VarId, InequalityOp, Parameters);

enum InequalityOp {
    SubtypeOp(SubtypeOp),
    OutlivesOp(OutlivesOp),
}

enum SubtypeOp {
    Subset,
    Superset,
}

enum OutlivesOp {
    Outlives,
    OutlivedBy,
}

enum ScalarId {
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

struct AdtId(String);
struct AliasId(String);
struct TraitId(String);
struct AssociatedTyId(String);
struct TyAliasId(String);
struct VarId(String);

struct KindedVarIds;

struct Generics(GenericParameters, WhereClauses);
struct GenericParameters(Vec<GenericParameter>);
struct GenericParameter(VarId, KindAndVariance);
struct KindAndVariance(ParameterKind, Variance);
enum Variance {
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
