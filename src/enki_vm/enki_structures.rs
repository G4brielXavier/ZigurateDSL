use std::collections::HashMap;

use crate::enki_vm::UdaDeclarationType;
use crate::enki_vm::EnkiInstruction;
use crate::enki_vm::SignalType;



#[derive(Debug, Clone)]
pub struct UDA {

    pub name: String,
    pub signal: String,
    pub value: UdaDeclarationType,
    pub is_nisa: bool

}














#[derive(Debug, Clone)]
pub struct LUNIG {
    pub id: String,
    pub data: HashMap<String, UdaDeclarationType>
}

















#[derive(Debug, Clone, PartialEq)]
pub struct UDA_EDIT {
    pub name: String,
    pub signal: SignalType,
    pub new_value: UdaDeclarationType
}








#[derive(Debug, Clone, PartialEq)]
pub struct LUNIG_EDIT {
    pub name: String,
    pub properties: Vec<String>,
    pub new_value: UdaDeclarationType
}










#[derive(Debug, Clone)]
pub struct ANKI {
    pub enki_instr: Vec<EnkiInstruction>
}










#[derive(Debug, Clone)]
pub struct MARU {
    pub path: String,
}









#[derive(Debug, Clone)]
pub struct GISH {
    pub id: String,
    pub enums: Vec<String>
}








#[derive(Debug, Clone)]
pub struct CALL_FUNCTION {
    pub obj_name: String,
    pub ident: String,
    pub args: Vec<String>
}









