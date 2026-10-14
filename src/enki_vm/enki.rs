
use std::{any::Any, path::PathBuf, println};

use dirs::picture_dir;

use crate::{core::{error::ZigurateError, utils::primi::{EXPR_Primi, SignalType}}, enki_vm::{EnkiInstruction, UdaDeclarationType, compiler::{convert_to_uda_declaration, extr_expr_string, get_expr_id}}};
use crate::enki_vm::compiler::extr_udatype_string;
use crate::core::zigurage_compiler_simple;
use crate::cli::get_enki_data;
use crate::tools::sita::Sita;

use std::collections::HashMap;

pub type UdaValue  = HashMap<String, UdaDeclarationType>;
pub type UdaData   = HashMap<String, UdaValue>;

pub type GishData  = HashMap<String, Vec<String>>;
pub type LunigData = HashMap<String, HashMap<String, UdaDeclarationType>>;





fn val_to_expr(val: String) -> EXPR_Primi {

    if let Ok(v) = val.parse::<bool>() {
        return EXPR_Primi::Boolean(v)
    }

    if let Ok(v) = val.parse::<i32>() {
        return EXPR_Primi::Integer(v)
    }

    if let Ok(v) = val.parse::<f32>() {
        return EXPR_Primi::Floating(v)
    }


    EXPR_Primi::String(val)

}




pub struct EnkiVM {
    instructions: Vec<EnkiInstruction>,

    uda_hash  : UdaData,
    lunig_hash: LunigData,
    gish_hash : GishData,

    sita: Sita,
    ip  : usize
}

impl EnkiVM {

    pub fn on(inst: Vec<EnkiInstruction>) -> Self {
        Self {
            instructions: inst,

            uda_hash  : HashMap::new(), 
            lunig_hash: HashMap::new(),
            gish_hash : HashMap::new(),

            sita: Sita::gibil(),
            ip  : 0
        }
    }
    
    pub fn run(&mut self) -> Result<(), ZigurateError> {

        while self.ip < self.instructions.len() {

            let instr = self.instructions[self.ip].clone();
            self.ip += 1;

            match instr {

                EnkiInstruction::INST_SILIM(val) => {
                    
                    let iden_signal = val.chars().next().unwrap();

                    if iden_signal == '!' {

                        let uda_key = val.as_str()[1..].to_string();

                        let splitted_key = uda_key.split_once("-");

                        let mut uda_captured;
                        let mut arr_idx: usize;

                        if let Some((k, idx)) = splitted_key {

                            arr_idx = idx.parse::<usize>().unwrap();

                            let msg = format!("Not found an uda called `{}`", &k);
                            uda_captured = self.uda_hash.get(&k.to_string())
                                .unwrap_or_else(|| panic!("{}`", ZigurateError::NotFound(msg)));

                        } else {

                            arr_idx = 888888 as usize;

                            let msg = format!("Not found an uda called `{}`", &uda_key);
                            uda_captured = self.uda_hash.get(&uda_key)
                                .unwrap_or_else(|| panic!("{}`", ZigurateError::NotFound(msg)));

                        }

                        let uda_value = uda_captured.get("value").unwrap();

                        match uda_value {
                            UdaDeclarationType::PathProperties(props) => {

                                // case1: if is a lunig

                                let from = &props[0];

                                let lunig_found = self.lunig_hash.get(from);

                                match lunig_found {
                                    Some(lunig) => {

                                        let prop = &props[1];
                                        let value = lunig.get(&*prop);
                                        
                                        match value {
                                            Some(val) => {

                                                let lunig_value = extr_udatype_string(val)?;
                                                println!("{}", lunig_value);
                                                continue;

                                            }
                                            None => {

                                                let gish_found_verifier = self.gish_hash.get(from);

                                                match gish_found_verifier {
                                                    Some(_) => {},
                                                    None => {
                                                        let msg = format!("Property `{}` not found in lunig `{}`", from, prop);
                                                        panic!("{}", ZigurateError::PropertyNotFound(msg));
                                                    }
                                                }
                                                
                                            }
                                        }

                                    }
                                    None => {}
                                }

                                // case2: if is a gish

                                let gish_found = self.gish_hash.get(from);

                                match gish_found {
                                    Some(gish) => {

                                        let prop = format!("#{}", &props[1]);

                                        let option_founded = gish.iter().find(|e| **e == prop);

                                        match option_founded {
                                            Some(v) => {
                                                let rest = &v[1..];
                                                println!("{}", rest);
                                                continue;
                                            }
                                            None => {
                                                let msg = format!("Option `{}` not found in gish called `{}`", prop, from);
                                                panic!("{}", ZigurateError::GishOptionNotFound(msg));
                                            }
                                        }

                                    }
                                    None => { 
                                        let msg = format!("Not found a gish called `{}`", from);
                                        panic!("{}", ZigurateError::NotFound(msg));
                                    }
                                }


                            }
                            UdaDeclarationType::Vector(vec_data) => {

                                if arr_idx < 888888 && arr_idx > vec_data.len() {
                                    let msg = format!("Array has {}, and index provided is {}. Limit exceeded.", vec_data.len(), arr_idx);
                                    panic!("{}", ZigurateError::ArrayIndexExceeded(msg));
                                }

                                let ele_in_vec = vec_data.get(arr_idx);

                                match ele_in_vec {
                                    Some(v) => {

                                        let id_expr = get_expr_id(v.clone());
                                        let uda_decl = convert_to_uda_declaration(v.clone());
                                        let data = extr_udatype_string(&uda_decl)?;

                                        match id_expr {

                                            "integer" => {
                                                println!("{:?}", data.parse::<i32>().unwrap());
                                            },
                                            "string" => {
                                                println!("{}", data)
                                            }
                                            "floating" => {
                                                println!("{:?}", data.parse::<f32>().unwrap())
                                            },
                                            "bool" => {
                                                println!("{}", data.parse::<bool>().unwrap())
                                            }

                                            &_ => {

                                            }
                                        }

                                    }
                                    None => {

                                    }
                                }

                            }
                            _ => {
                                let msg_uda = extr_udatype_string(&uda_value)?;
                                println!("{}", msg_uda);
                            }
                        }

                    } else {
                        println!("{}", val);
                    }

                },






                EnkiInstruction::INST_SILIM_PROP(val) => {

                    let k_start = val[0].to_string();
                    
                    if val.is_empty() {
                        println!();
                        continue;
                    }

                    let rest = &val[1..];

                    // Verify if lunig exist 
                    let lunig_founded = self.lunig_hash.get(&k_start);

                    match lunig_founded {
                        Some(lunig)  => {

                            for (i, prop) in rest.iter().enumerate() {

                                if i >= rest.len() {
                                    break;
                                }

                                let data = lunig.get(prop);
                                

                                match data {
                                    Some(v) => {
                                        
                                        match v {
                                            _ => {
                                                let val = extr_udatype_string(v)?;
                                                println!("{}", val);
                                                break
                                            }
                                        }

                                    },
                                    None => {
                                        let msg = format!("Not found property `{}` in lunig `{}`.", k_start, prop);
                                        panic!("{}", ZigurateError::PropertyNotFound(msg))
                                    }
                                }

                            }

                        },
                        None => {
                            let msg = format!("A lunig called `{}` not found or not exists.", k_start);
                            panic!("{}", ZigurateError::NotFound(msg))
                        }
                    }

                },






                EnkiInstruction::INST_UDA_DECL(uda) => {

                    let uda_founded = self.uda_hash.get(&uda.name);

                    match uda_founded {
                        Some(_) => {
                            let msg = format!("An Uda called `{}` already exists.", uda.name);
                            panic!("{}", ZigurateError::AlreadyExists(msg));
                        },
                        None => {
                            let mut uda_value = HashMap::new();

                            match uda.value {

                                UdaDeclarationType::ArrayIndexer(arr) => {

                                    let uda_found = self.uda_hash.get(&arr[0]);
                                    
                                    match uda_found {

                                        Some(uda_f) => {

                                            let values = uda_f.get("value").unwrap();

                                            if let UdaDeclarationType::Vector(vec) = values {

                                                let idx = arr[1].parse::<usize>().unwrap();

                                                let vec_value = convert_to_uda_declaration(vec[idx].clone());

                                                uda_value.insert("value".to_string(), vec_value);
                                                uda_value.insert("is_nisas".to_string(), UdaDeclarationType::Boolean(uda.is_nisa));
                                                
                                                self.uda_hash.insert(uda.name, uda_value);

                                            }

                                        }

                                        None => {}

                                    }

                                    let lunig_found = self.lunig_hash.get(&arr[0]);

                                    match lunig_found {

                                        Some(lunig_f) => {

                                        },

                                        None => {}

                                    }

                                }

                                _ => {
                                    uda_value.insert("value".to_string(), uda.value);
                                    uda_value.insert("is_nisa".to_string(), UdaDeclarationType::Boolean(uda.is_nisa));

                                    self.uda_hash.insert(uda.name, uda_value);
                                }

                            }
                        }
                    }

                }







                EnkiInstruction::INST_UDA_EDIT(edit_uda) => {

                    let uda_founded = self.uda_hash.get_mut(&edit_uda.name);

                    match uda_founded {
                        Some(uda) => {

                            match edit_uda.signal {

                                SignalType::NewValue => {
                                    
                                    let is_nisa = if let UdaDeclarationType::Boolean(v) = uda.get("is_nisa").unwrap() {
                                        *v
                                    } else {
                                        false
                                    };

                                    if is_nisa {

                                        let uda_val = uda.get_mut("value").unwrap();
                                        
                                        match uda_val {
                                            UdaDeclarationType::PathProperties(pp) => {

                                                let from = &pp[0];
                                                let lunig_found = self.lunig_hash.get_mut(from);

                                                match lunig_found {
                                                    Some(lunig) => {

                                                        let prop = &pp[1];
                                                        let lunig_prop = lunig.get_mut(prop);

                                                        match lunig_prop {
                                                            Some(prop) => {

                                                                *prop = edit_uda.new_value;
                                                                
                                                            },
                                                            None => {
                                                                let msg = format!("Not found property `{}` in lunig `{}`.", prop, from);
                                                                panic!("{}", ZigurateError::PropertyNotFound(msg))          
                                                            }
                                                        }

                                                    },
                                                    None => {
                                                        let msg = format!("A lunig called `{}` not found or not exists.", from);
                                                        panic!("{}", ZigurateError::NotFound(msg))
                                                    }
                                                }
                                            
                                            },
                                            _ => {
                                                let msg = format!("`{}` is using modifier `@nisa` of wrong way. It's only used to LunigProperties, like: `lunig_obj.name`", &edit_uda.name);
                                                panic!("{}", ZigurateError::SyntaxError(msg));
                                            }
                                        }
                                    
                                    } else {
                                        let uda_val = uda.get_mut("value").unwrap();
                                        *uda_val = edit_uda.new_value;
                                    }

                                }

                                _ => {
                                    let msg = format!("Occurred an syntax error to change `{}`, verify the signal (=, +=, -=) used.", edit_uda.name);
                                    panic!("{}", ZigurateError::SyntaxError(msg));
                                }
                            }

                        },
                        None => {
                            let msg = format!("An uda called `{}` not exist or not found.", edit_uda.name);
                            panic!("{}", ZigurateError::NotFound(msg));
                        }
                    }

                }







                EnkiInstruction::INST_LUNIG_EDIT(edit_lunig) => {

                    let lunig_founded = self.lunig_hash.get_mut(&edit_lunig.name);

                    match lunig_founded {

                        Some(lunig) => {

                            if edit_lunig.properties.len() == 2 {

                                let prop = &edit_lunig.properties[1];
                                let prop_found = lunig.get_mut(prop);


                                match prop_found {

                                    Some(value) => {

                                        *value = edit_lunig.new_value

                                    }
                                    None => {
                                        lunig.insert(prop.clone(), edit_lunig.new_value);
                                    }

                                }


                            
                            } else {
                                let msg = format!("lunig can only accept one propertie, example: `obj.name`\n -- To use more, see about `tud` in documentation.");
                                panic!("{}", ZigurateError::PropertyLimitExceeded(msg));
                            }

                        }

                        None => {
                            let msg = format!("A lunig called \"{}\" not exist or not found.", edit_lunig.name);
                            panic!("{}", ZigurateError::NotFound(msg));
                        }

                    }

                }







                EnkiInstruction::INST_LUNIG_DECL(lunig) => {

                    if self.lunig_hash.contains_key(&lunig.id) {
                        let msg = format!("A lunig called `{}` already exists.", lunig.id);
                        panic!("{}", ZigurateError::AlreadyExists(msg))
                    }

                    self.lunig_hash.insert(lunig.id, lunig.data);

                }






                // insert a new instruction in pipe-line in execution-time
                EnkiInstruction::INST_ANKI(anki) => {

                    let instruct = anki.enki_instr;
                    
                    if instruct.len() == 1 {

                        let next_ip = if self.ip > self.instructions.len() {
                            self.instructions.len()
                        } else {
                            self.ip as usize
                        };
                        
                        self.instructions.insert(next_ip, instruct[0].clone());
                        continue;

                    }
                    
                }






                // remove an uda in execution-time (anki.:kud())
                EnkiInstruction::INST_CUT_UDA(data) => {
  
                    if data.len() == 1 {
                        
                        let uda_comp_k = data[0].to_string();
                        let signal = uda_comp_k.chars().next().unwrap();

                        if signal == '!' {

                            let rest = uda_comp_k[1..].to_string();

                            self.uda_hash.remove(&rest);
                            continue;
                        
                        }

                    } else {

                        for uda_k in data.iter() {

                            let signal = uda_k.chars().next().unwrap();

                            if signal == '!' {

                                let rest = uda_k[1..].to_string();

                                self.uda_hash.remove(&rest);
                            
                            }

                        }

                        continue;

                    }

                }
            
            




                // remove a lunig in execution-time (with anki.:gaz())
                EnkiInstruction::INST_GAZ_LUNIG(data) => {

                    if data.len() == 1 {

                        let lunig_comp_k = data[0].to_string();
                        let signal = lunig_comp_k.chars().next().unwrap();

                        if signal == '!' {

                            let rest = lunig_comp_k[1..].to_string();

                            self.lunig_hash.remove(&rest);
                            continue;

                        }

                    } else {

                        for lunig_k in data.iter() {

                            let signal = lunig_k.chars().next().unwrap();

                            if signal == '!' {
                                let rest = lunig_k[1..].to_string();
                                self.lunig_hash.remove(&rest);
                            }

                        }

                        continue;

                    }

                }







                EnkiInstruction::INST_MARU(maru) => {

                    // read the path answered
                    let curr_path = self.sita.current_dir()
                        .map_err(|e| ZigurateError::IOError(e))?;

                    let forma_path = maru.path.replace("/", "\\");
                    let enki_project_data =  get_enki_data(&self.sita, &curr_path)?;

                    let sub_src: Vec<&str> = enki_project_data.main.split("/").collect();
                    let path_file = curr_path.join(sub_src[0]).join(PathBuf::from(forma_path));

                    let file_content = self.sita.read_file(path_file)
                        .map_err(|e| ZigurateError::IOError(e))?;



                    // get the file content as bytes
                    let bytes_content = file_content.as_bytes();

                    let curr_ip = self.ip;



                    // deliver the byte content to EnkiCompiler read
                    let new_enki_instructions = zigurage_compiler_simple(bytes_content)?;



                    // insert the newest instructions in main list's instructions 
                    for inst in new_enki_instructions.iter().rev() {
                        self.instructions.insert(curr_ip, inst.clone());
                    }

                    // for i in self.instructions.iter() {
                    //     println!("{:?}", i);
                    // }

                    self.run()?;

                }
            
            





                EnkiInstruction::INST_GISH(gish) => {

                    let gish_found = self.gish_hash.get(&gish.id);

                    match gish_found {
                    
                        Some(_) => {
                            let msg = format!("Already exist a gish called `{}`", gish.id);
                            panic!("{}", ZigurateError::AlreadyExists(msg));
                        }
                        None => {
                            self.gish_hash.insert(gish.id.to_string(), gish.enums);
                        }

                    }

                }

            
            



                
                EnkiInstruction::INST_CALL_FNC(fnc) => {

                    let splitted_identifier = fnc.ident.split_once("_");

                    let uda_found = self.uda_hash.get_mut(&fnc.obj_name);
                    let lunig_found = self.lunig_hash.get_mut(&fnc.obj_name);


                    if let Some((t_from, t_act)) = splitted_identifier {

                        match t_from {

                            "array" => {

                                match uda_found {

                                    Some(val) => {
                                        
                                        let is_nisa = if let UdaDeclarationType::Boolean(v) = val.get("is_nisa").unwrap() {
                                            *v
                                        } else {
                                            false
                                        };

                                        if !is_nisa {

                                            let uda_val = val.get_mut("value").unwrap();

                                            match uda_val {

                                                UdaDeclarationType::Vector(vec) => {

                                                    match t_act {

                                                        "push-item" => {

                                                            let val_to_push = fnc.args[0].to_string();
                                                            let expr_val = val_to_expr(val_to_push);

                                                            vec.push(expr_val);
                                                        
                                                        },

                                                        "pop-last-item" => {

                                                            vec.pop();
                                                        
                                                        },

                                                        "pop-item" => {

                                                            let idx_val = fnc.args[0].parse::<usize>().unwrap();
                                                            
                                                            if idx_val > vec.len() {
                                                                let msg = format!("Array has {}, and index provided is {}. Limit exceeded.", vec.len(), idx_val);
                                                                panic!("{}", ZigurateError::ArrayIndexExceeded(msg));
                                                            }

                                                            vec.remove(idx_val);
                                                        
                                                        },

                                                        "clear" => {

                                                            vec.clear();

                                                        },

                                                        &_ => {

                                                        }

                                                    }
                                                    
                                                }

                                                UdaDeclarationType::PathProperties(pp) => {

                                                    let lunig_found = self.lunig_hash.get_mut(&pp[0]);

                                                    match lunig_found {

                                                        Some(lunig) => {

                                                            let property_found = lunig.get_mut(&pp[1]);

                                                            match property_found {

                                                                Some(prop) => {

                                                                    match prop {
                                                                        UdaDeclarationType::Vector(vec) => {

                                                                            let msg = format!("You are trying to change a immutable Vector that's in a `lunig`. use `@nisa` to become it mutable.");
                                                                            panic!("{}", ZigurateError::PropertyNotFound(msg))

                                                                        }
                                                                        _ => {
                                                                            let msg = format!("Expected an Array in property `{}`. Got {:?}", pp[1], prop);
                                                                            panic!("{}", ZigurateError::PropertyNotFound(msg))
                                                                        }
                                                                    }
                                                                }

                                                                None => {
                                                                    let msg = format!("Not found a property called `{}` in lunig `{}`.", pp[1], pp[0]);
                                                                    panic!("{}", ZigurateError::PropertyNotFound(msg))
                                                                }

                                                            }

                                                        }

                                                        None => {
                                                            let msg = format!("A lunig called \"{}\" not exist or not found.", pp[0]);
                                                            panic!("{}", ZigurateError::NotFound(msg));
                                                        }

                                                    }

                                                    
                                                }


                                                _ => {
                                                    let msg = format!("Expect an Array. Got {:?}", uda_val);
                                                    panic!("{}", ZigurateError::GotWrongTypeValue(msg));
                                                }
                                            }

                                        } else {

                                            let uda_val = val.get_mut("value").unwrap();

                                            match uda_val {

                                                UdaDeclarationType::Vector(vec) => {

                                                    match t_act {

                                                        "push-item" => {

                                                            let val_to_push = fnc.args[0].to_string();
                                                            let expr_val = val_to_expr(val_to_push);

                                                            vec.push(expr_val);
                                                        
                                                        },

                                                        "pop-last-item" => {

                                                            vec.pop();
                                                        
                                                        },

                                                        "pop-item" => {

                                                            let idx_val = fnc.args[0].parse::<usize>().unwrap();
                                                            
                                                            if idx_val > vec.len() {
                                                                let msg = format!("Array has {}, and index provided is {}. Limit exceeded.", vec.len(), idx_val);
                                                                panic!("{}", ZigurateError::ArrayIndexExceeded(msg));
                                                            }

                                                            vec.remove(idx_val);
                                                        
                                                        },

                                                        "clear" => {

                                                            vec.clear();

                                                        },

                                                        &_ => {

                                                        }

                                                    }
                                                    
                                                }

                                                UdaDeclarationType::PathProperties(pp) => {

                                                    let lunig_found = self.lunig_hash.get_mut(&pp[0]);

                                                    match lunig_found {

                                                        Some(lunig) => {

                                                            let property_found = lunig.get_mut(&pp[1]);

                                                            match property_found {

                                                                Some(prop) => {

                                                                    match prop {
                                                                        UdaDeclarationType::Vector(vec) => {

                                                                            match t_act {

                                                                                "push-item" => {

                                                                                    let val_to_push = fnc.args[0].to_string();
                                                                                    let expr_val = val_to_expr(val_to_push);

                                                                                    vec.push(expr_val);
                                                                                
                                                                                },

                                                                                "pop-last-item" => {

                                                                                    vec.pop();
                                                                                
                                                                                },

                                                                                "pop-item" => {

                                                                                    let idx_val = fnc.args[0].parse::<usize>().unwrap();
                                                                                    
                                                                                    if idx_val > vec.len() {
                                                                                        let msg = format!("Array has {}, and index provided is {}. Limit exceeded.", vec.len(), idx_val);
                                                                                        panic!("{}", ZigurateError::ArrayIndexExceeded(msg));
                                                                                    }

                                                                                    vec.remove(idx_val);
                                                                                
                                                                                },

                                                                                "clear" => {

                                                                                    vec.clear();

                                                                                },

                                                                                &_ => {

                                                                                }

                                                                            }

                                                                        }
                                                                        _ => {
                                                                            let msg = format!("Expected an Array in property `{}`. Got {:?}", pp[1], prop);
                                                                            panic!("{}", ZigurateError::PropertyNotFound(msg))
                                                                        }
                                                                    }
                                                                }

                                                                None => {
                                                                    let msg = format!("Not found a property called `{}` in lunig `{}`.", pp[1], pp[0]);
                                                                    panic!("{}", ZigurateError::PropertyNotFound(msg))
                                                                }

                                                            }

                                                        }

                                                        None => {
                                                            let msg = format!("A lunig called \"{}\" not exist or not found.", pp[0]);
                                                            panic!("{}", ZigurateError::NotFound(msg));
                                                        }

                                                    }

                                                    
                                                }

                                                _ => {
                                                    let msg = format!("Expect an Array. Got {:?}", uda_val);
                                                    panic!("{}", ZigurateError::GotWrongTypeValue(msg));
                                                }
                                            }

                                        }

                                    }

                                    None => {
                                        let msg = format!("An uda called `{}` not exist or not found.", fnc.obj_name);
                                        panic!("{}", ZigurateError::NotFound(msg));
                                    }

                                }

                            }

                            &_ => {

                            }

                        }

                    } 

                }

            }


        }
        Ok(())

    }

    pub fn match_fnc_acts(&mut self, t_act: &'static str) -> Result<(), ZigurateError> {

        match t_act {

            "push-item" => {



            }

            &_ => {

            }

        }

        Ok(())
        
    }

}