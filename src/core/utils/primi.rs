use crate::core::utils::structures::{
    TEMEM_Anki, TEMEM_EditLunig, TEMEM_EditVariable, TEMEM_Function, TEMEM_Gish, TEMEM_Lunig, TEMEM_Maru, TEMEM_ObjFunction, TEMEM_TempVariable
};


#[derive(Debug, PartialEq, Clone)]
pub enum Primi {
    
    Identifier,

    Symbol,
    Integer,
    Floating,
    String,
    Boolean,

    Kwd_Function,
    Kwd_Variable,
    kwd_Lunig,
    Kwd_Simple,
    Kwd_Anki,
    Kwd_Maru,
    Kwd_Gish,
    Kwd_Modificators,

}



#[derive(Debug, PartialEq, Clone)]
pub enum STRT_Zilamma {

    CallFunction(TEMEM_Function),
    ObjCallFunction(TEMEM_ObjFunction),

    DeclUda(TEMEM_TempVariable),
    DeclLunig(TEMEM_Lunig),

    Anki(TEMEM_Anki),
    Maru(TEMEM_Maru),
    Gish(TEMEM_Gish),

    EditLunig(TEMEM_EditLunig),
    EditUda(TEMEM_EditVariable)

}



#[derive(Debug, PartialEq, Clone)]
pub enum EXPR_Primi {

    String(String),
    Integer(i32),
    Boolean(bool),
    Floating(f32),
    Symbol(char),
    Identifier(String),

    ArrayIndexer(Vec<String>),

    Vector(Vec<EXPR_Primi>),
    PathProperties(Vec<String>),

    KWD_SIMPLE(String),
    Empty

}



#[derive(Debug, PartialEq, Clone)]
pub enum SignalType {
    NewValue,
    AddTo,
    SubTo
}