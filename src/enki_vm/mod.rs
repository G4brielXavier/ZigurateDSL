use std::collections::HashMap;

use crate::{core::utils::primi::{EXPR_Primi, SignalType}, enki_vm::enki_structures::CALL_FUNCTION};

pub mod compiler;
pub mod enki;
pub mod enki_structures;


use enki_structures::{
    ANKI,

    LUNIG,
    LUNIG_EDIT,

    UDA,
    UDA_EDIT,

    MARU,
    GISH

};









#[derive(Debug, Clone, PartialEq)]
pub enum UniqueTypedVectors {
    VecString(Vec<String>),
    VecInteger(Vec<i32>),
    VecFloating(Vec<f32>),
    VecBoolean(Vec<bool>)
}








#[derive(Debug, Clone, PartialEq)]
pub enum UdaDeclarationType {
    String(String),
    Integer(i32),
    Floating(f32),
    Boolean(bool),
    Vector(Vec<EXPR_Primi>),
    ArrayIndexer(Vec<String>),
    PathProperties(Vec<String>)
}







#[derive(Debug, Clone)]
pub enum EnkiInstruction {

    INST_SILIM(String), // silim()
    INST_SILIM_PROP(Vec<String>), // silim(item.prop)

    INST_UDA_DECL(UDA), // uda name = value
    
    INST_CUT_UDA(Vec<String>), // anki.:kud(uda_name)
    INST_GAZ_LUNIG(Vec<String>), // anki.:gaz(lunig_name)

    INST_UDA_EDIT(UDA_EDIT), // uda_name = new_value
    INST_LUNIG_EDIT(LUNIG_EDIT), // lunig.prop = new_value

    INST_LUNIG_DECL(LUNIG), // lunig "Name" as name
    
    INST_ANKI(ANKI), // anki.:func()

    INST_MARU(MARU), // maru "./" ak id

    INST_GISH(GISH), // gish GishName: - GISH

    INST_CALL_FNC(CALL_FUNCTION)

}