use crate::{core::utils::primi::EXPR_Primi, enki_vm::UdaDeclarationType};
use crate::core::utils::primi::{STRT_Zilamma, SignalType};

use std::collections::HashMap;









#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_Function {
    pub name: String,
    pub args: Vec<EXPR_Primi>
}










#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_TempVariable {

    pub name: String,
    pub signal: String,
    pub value: EXPR_Primi,
    pub is_nisa: bool

}









#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_EditVariable {

    pub name: String,
    pub signal: SignalType,
    pub value: EXPR_Primi

}







#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_Lunig {
    pub id: String,
    pub data: HashMap<String, UdaDeclarationType>,
}









#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_EditLunig {
    pub lunig_id: String,
    pub path_prop: EXPR_Primi,
    pub new_value: EXPR_Primi
}










#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_Anki {
    pub fnc: Box<STRT_Zilamma>
}












#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_Maru {
    pub path: String,
}











#[derive(Debug, PartialEq, Clone)]
pub struct TEMEM_Gish {
    pub id: String,
    pub enums: Vec<EXPR_Primi>
}