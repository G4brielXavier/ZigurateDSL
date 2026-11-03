#![allow(non_camel_case_types)]

use crate::core::utils::structures::{
    
    TEMEM_Anki, 
    TEMEM_Function,
    TEMEM_ObjFunction, 

    TEMEM_TempVariable,
    TEMEM_EditVariable, 
    
    TEMEM_Lunig, 
    TEMEM_EditLunig, 
        
    TEMEM_Maru, 
    TEMEM_Gish, 
    TEMEM_Suma,

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
    Kwd_ComparationOP,
    Kwd_Variable,
    kwd_Lunig,
    Kwd_Simple,
    Kwd_Anki,
    Kwd_Maru,
    Kwd_Gish,
    Kwd_Suma,
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

    Suma(TEMEM_Suma),

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
    DumuStructure(Vec<EXPR_Primi>),
    Dumune(Vec<EXPR_Primi>),
    Zilamma(Box<STRT_Zilamma>),

    OPT_AND,
    OPT_OR,

    OPT_IS,
    OPT_IS_NOT,
    OPT_IS_MORE,
    OPT_IS_LESS,
    OPT_IS_MORE_EQ,
    OPT_IS_LESS_EQ,


    KWD_SIMPLE(String),
    Empty

}



#[derive(Debug, PartialEq, Clone)]
pub enum SignalType {
    NewValue
}