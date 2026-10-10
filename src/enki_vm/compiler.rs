
use core::panic;
use std::collections::HashMap;
use std::fmt::format;
use std::{convert, println};
use std::any::Any;

use crate::core::error::ZigurateError;
use crate::core::utils::primi::{EXPR_Primi, STRT_Zilamma};

use crate::enki_vm::{EnkiInstruction, UdaDeclarationType, UniqueTypedVectors};


use crate::enki_vm::{
    UDA,
    LUNIG,
    UDA_EDIT,
    ANKI,
    LUNIG_EDIT,
    MARU,
    GISH
};


pub fn get_expr_id(expr: EXPR_Primi) -> &'static str {
    match expr {
        EXPR_Primi::Boolean(_) => "bool",
        EXPR_Primi::String(_) => "string",
        EXPR_Primi::Integer(_) => "integer",
        EXPR_Primi::Floating(_) => "floating",
        _ => ""
    }
}


pub fn extr_expr_string(expr: EXPR_Primi, categ: &str) -> String {

    match expr {
        
        EXPR_Primi::String    (e) => e,
        EXPR_Primi::Integer   (i)    => i.to_string(),
        EXPR_Primi::Floating  (f)    => f.to_string(),
        EXPR_Primi::Identifier(t) => {
            if categ == "uda" || categ == "lunig" {
                return format!("!{}", t)
            } else if categ == "gish" {
                return format!("#{}", t)
            } else {
                return format!("!{}", t)
            }
        },
        EXPR_Primi::Boolean   (t)   => format!("{}", t),

        _ => "".to_string()
    }

}


pub fn extr_udatype_string(udadecl: &UdaDeclarationType) -> Result<String, ZigurateError> {

    match udadecl {
        
        UdaDeclarationType::String  (v) => return Ok(v.to_string()),
        UdaDeclarationType::Integer (v)    => return Ok(v.to_string()),
        UdaDeclarationType::Floating(v)    => return Ok(v.to_string()),
        UdaDeclarationType::Boolean (v)   => return Ok(v.to_string()),
        UdaDeclarationType::PathProperties(pp) => return Ok(format!("{}.{}", pp[0], pp[1])),
        UdaDeclarationType::Vector(vec)  => {
            
            let mut temporary_vec_expr = Vec::with_capacity(vec.len());

            if vec.is_empty() {
                return Ok("[]".to_string())
            }

            let pattern_type = get_expr_id(vec[0].clone());

            for expr in vec.iter() {

                let patt_curr = get_expr_id(expr.clone());

                if pattern_type == patt_curr {
                    let decl_enki_type = convert_to_uda_declaration(expr.clone());
                    temporary_vec_expr.push(decl_enki_type);
                } else {
                    let msg = format!("It's mixed. Expected type `{}`, but found `{}`. Try obey the type's value of first value in list.", pattern_type, patt_curr);
                    return Err(ZigurateError::VectorTypesValueIsMixed(msg));
                }

            }

            let real_values: Result<UniqueTypedVectors, ZigurateError> = match &temporary_vec_expr[0] {
                
                UdaDeclarationType::Boolean(_) => {
                    let mut v = Vec::new();
                    for it in &temporary_vec_expr {
                        if let UdaDeclarationType::Boolean(val) = it { v.push(*val); }
                        else { return Err(ZigurateError::SyntaxError("".into())); }
                    }
                    Ok(UniqueTypedVectors::VecBoolean(v))
                },

                UdaDeclarationType::Integer(_) => {
                    let mut v = Vec::new();
                    for it in &temporary_vec_expr {
                        if let UdaDeclarationType::Integer(val) = it { v.push(*val); }
                        else { return Err(ZigurateError::SyntaxError("".into())); }
                    }
                    Ok(UniqueTypedVectors::VecInteger(v))
                },

                UdaDeclarationType::Floating(_) => {
                    let mut v = Vec::new();
                    for it in &temporary_vec_expr {
                        if let UdaDeclarationType::Floating(val) = it { v.push(*val); }
                        else { return Err(ZigurateError::SyntaxError("".into())); }
                    }
                    Ok(UniqueTypedVectors::VecFloating(v))
                },

                UdaDeclarationType::String(_) => {
                    let mut v = Vec::new();
                    for it in &temporary_vec_expr {
                        if let UdaDeclarationType::String(val) = it { v.push(val.clone()); }
                        else { return Err(ZigurateError::SyntaxError("".into())); }
                    }
                    Ok(UniqueTypedVectors::VecString(v))
                },

                _ => Err(ZigurateError::TypeNotSupported("Type value not supported by Vectors".into()))

            };

            match real_values.unwrap() {
                UniqueTypedVectors::VecBoolean(v) => Ok(format!("{:?}", v)),
                UniqueTypedVectors::VecFloating(v) => Ok(format!("{:?}", v)),
                UniqueTypedVectors::VecInteger(v) => Ok(format!("{:?}", v)),
                UniqueTypedVectors::VecString(v) => Ok(format!("{:?}", v)),
            }

        }

        _ => Ok("".to_string())
    }

}


pub fn convert_to_uda_declaration(expr: EXPR_Primi) -> UdaDeclarationType {

    match expr {
        EXPR_Primi::String        (e)      => UdaDeclarationType::String(e),
        EXPR_Primi::Integer       (i)         => UdaDeclarationType::Integer(i),
        EXPR_Primi::Floating      (f)         => UdaDeclarationType::Floating(f),
        EXPR_Primi::Boolean       (f)        => UdaDeclarationType::Boolean(f),
        EXPR_Primi::PathProperties(f) => UdaDeclarationType::PathProperties(f),
        EXPR_Primi::Vector        (vec) => UdaDeclarationType::Vector(vec),

        _ => UdaDeclarationType::String("".to_string())
    }

}














pub fn enki_compiler(ast: Vec<STRT_Zilamma>) -> Result<Vec<EnkiInstruction>, ZigurateError> {

    let mut enkienes = Vec::new();

    for step in ast.iter() {

        match step {
            
            STRT_Zilamma::CallFunction(fnc) => {

                match fnc.name.as_str() {
                    
                    "silim" => {
                        let args = &fnc.args;
                    
                        if args.len() == 0 {
                            let enki_sar = EnkiInstruction::INST_SILIM(" ".to_string());
                            enkienes.push(enki_sar);
                        }

                        if args.len() == 1 {

                            let arg_i = args[0].clone();

                            match arg_i {

                                EXPR_Primi::PathProperties(pp) => {

                                    let enki_sar = EnkiInstruction::INST_SILIM_PROP(pp);
                                    enkienes.push(enki_sar);

                                }

                                _ => {

                                    let arg = extr_expr_string(arg_i, "uda");
                                    let enki_sar = EnkiInstruction::INST_SILIM(arg);
                                    enkienes.push(enki_sar);

                                }

                            }

                        }
                    }

                    "kud" => {

                        let args = &fnc.args;

                        if args.len() == 1 {

                            let arg = args[0].clone();
                            let expr = extr_expr_string(arg, "uda");

                            let enki_sar = EnkiInstruction::INST_CUT_UDA(vec![expr]);
                            enkienes.push(enki_sar);

                        } else {



                        }

                    }

                    "gaz" => {

                        let args = &fnc.args;

                        if args.len() == 1 {

                            let arg = args[0].clone();
                            let expr = extr_expr_string(arg, "lunig");

                            let enki_sar = EnkiInstruction::INST_GAZ_LUNIG(vec![expr]);
                            enkienes.push(enki_sar);

                        }

                    }

                    &_ => {
                        let msg = format!("Unknown function name `{}`", fnc.name.to_string());
                        panic!("{}", ZigurateError::UnknownCallFunctionName(msg));
                    }
                }

            }

            STRT_Zilamma::DeclUda(uda) => {
                
                let uda_name = &uda.name;
                let uda_signal = &uda.signal;
                let uda_is_nisa = &uda.is_nisa;

                let uda_value = convert_to_uda_declaration(uda.value.clone());

                let uda_m = UDA {
                    name: uda_name.to_string(),
                    signal: uda_signal.to_string(),
                    value: uda_value,
                    is_nisa: *uda_is_nisa
                };

                let enki_sar = EnkiInstruction::INST_UDA_DECL(uda_m);
                enkienes.push(enki_sar);

            }

            STRT_Zilamma::DeclLunig(lunig) => {

                let lunig_m = LUNIG {
                    id: lunig.id.clone(),
                    data: lunig.data.clone()
                };

                let enki_sar = EnkiInstruction::INST_LUNIG_DECL(lunig_m);
                enkienes.push(enki_sar);

            }

            STRT_Zilamma::EditUda(edit) => {

                let edit_uda_m = UDA_EDIT {
                    name: edit.name.clone(),
                    signal: edit.signal.clone(),
                    new_value: convert_to_uda_declaration(edit.value.clone())
                };

                let enki_sar = EnkiInstruction::INST_UDA_EDIT(edit_uda_m);
                enkienes.push(enki_sar);

            }

            STRT_Zilamma::EditLunig(edit) => {

                match &edit.path_prop {
                    EXPR_Primi::PathProperties(pp) => {

                        let edit_lunig_m = LUNIG_EDIT {
                            name: edit.lunig_id.clone(),
                            properties: pp.clone(),
                            new_value: convert_to_uda_declaration(edit.new_value.clone())
                        };

                        let enki_sar = EnkiInstruction::INST_LUNIG_EDIT(edit_lunig_m);
                        enkienes.push(enki_sar);

                    },
                    _ => {
                        return Err(ZigurateError::PropertiesExpected("".to_string()))
                    }
                }

            }

            STRT_Zilamma::Anki(anki) => {

                let anki_zilamma = *anki.fnc.clone();

                let fnc_intr_anki = enki_compiler(vec![anki_zilamma])?;

                let anki_m = ANKI { enki_instr: fnc_intr_anki };
                let enki_sar = EnkiInstruction::INST_ANKI(anki_m);

                enkienes.push(enki_sar)

            }

            STRT_Zilamma::Maru(maru) => {

                let maru_m = MARU { path: maru.path.clone() };
                let enki_sar = EnkiInstruction::INST_MARU(maru_m);

                enkienes.push(enki_sar);

            }

            STRT_Zilamma::Gish(gish) => {

                let mut gish_vec = Vec::new();

                for i in gish.enums.iter() {
                    gish_vec.push(extr_expr_string(i.clone(), "gish"))
                }

                let gish_m = GISH {
                    id: gish.id.clone(),
                    enums: gish_vec
                };

                let enki_sar = EnkiInstruction::INST_GISH(gish_m);
                enkienes.push(enki_sar);

            }

        }

    }

    Ok(enkienes)

}