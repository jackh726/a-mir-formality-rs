#![allow(unused)]

use crate::ty_grammar::*;

pub struct DeclProgram(CrateDecls, CrateId);

pub struct CrateDecls(Vec<CrateDecl>);
pub struct CrateDecl(CrateId, CrateContents);
pub struct CrateContents(Vec<CrateItemDecl>);
pub enum CrateItemDecl {
    AdtDecl(AdtDecl),
    TraitDecl(TraitDecl),
    TraitImplDecl(TraitImplDecl),
    ConstDecl(ConstDecl),
}

pub struct AdtDecl(AdtId, AdtContents);
pub struct AdtContents(AdtKind, KindedVarIds, WhereClauses, AdtVariants);
pub struct AdtVariants(Vec<AdtVariant>);
pub enum AdtKind {
    Struct,
    Enum,
    Union,
}
pub struct AdtVariant(VariantId, FieldDecls);

pub struct FieldDecls(Vec<FieldDecl>);
pub struct FieldDecl(FieldId, Ty);

pub struct TraitDecl(TraitId, TraitContents);
pub struct TraitContents(KindedVarIds, WhereClauses, TraitItems);

pub struct TraitItems(Vec<TraitItem>);
pub enum TraitItem {}

pub struct TraitImplDecl(KindedVarIds, TraitRef, WhereClauses, ImplItems);
pub struct ConstDecl(ConstId, ConstContents);
pub struct ConstContents(KindedVarIds, WhereClauses, Ty);

pub struct ImplItems(Vec<ImplItem>);
pub enum ImplItem {}

pub struct CrateId(String);
pub struct TraitImplId(String);
pub struct ConstId(String);
pub struct VariantId(String);
pub struct FieldId(String);
