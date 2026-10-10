use std::collections::HashMap;
use std::{print, println};

use colored::Colorize;

use crate::core::utils::primi::Primi::Kwd_Modificators;
use crate::core::{error::ZigurateError};

use crate::core::utils::token::Token;
use crate::core::utils::primi::{EXPR_Primi, Primi, SignalType};
use crate::core::utils::primi::STRT_Zilamma;


use crate::core::utils::structures::{
    TEMEM_Anki, 
    TEMEM_EditLunig, 
    TEMEM_EditVariable, 
    TEMEM_Function, 
    TEMEM_Lunig, 
    TEMEM_TempVariable, 
    TEMEM_Maru,
    TEMEM_Gish
};


use crate::enki_vm::UdaDeclarationType;



use std::str::from_utf8;





pub struct AST<'a> {
    pub i: usize,
    pub tokens: Vec<Token<'a>>
}


impl<'a> AST<'a> {

    pub fn open(tokens: Vec<Token<'a>>) -> Self {
        Self { i: 0, tokens }
    }


    pub fn step(&mut self) -> &Token<'a> {

        let token = &self.tokens[self.i];
        self.i += 1;
        token
    
    }


    pub fn see(&self) -> Option<&Token<'a>> {

        if self.i < self.tokens.len() {
            Some(&self.tokens[self.i])
        } else {
            None
        }

    }


    pub fn see_next(&self) -> Option<&Token<'a>> {

        if self.i + 1 < self.tokens.len() {
            Some(&self.tokens[self.i + 1])
        } else {
            None
        }

    }






    pub fn parse_list(&mut self) -> Result<EXPR_Primi, ZigurateError> {

        let mut data = Vec::new();

        while self.see().is_some() && from_utf8(self.see().unwrap().value).unwrap().chars().next().unwrap() != ']' {

            let virg_sepa = self.see();

            match virg_sepa {

                Some(v) => {
                    match v.primi {
                        Primi::Symbol => {

                            let v = from_utf8(v.value)?.chars().next().unwrap();

                            if v == ',' {
                                self.step();
                            } else {
                                break
                            }

                        }
                        _ => {
                            let val_tk = self.parse_expr()?;
                            data.push(val_tk);
                            continue;
                        }
                    }
                }

                None => {
                    break
                }

            }

        }


        self.step();

        let struct_data = EXPR_Primi::Vector(data);
        return Ok(struct_data)

    }




    pub fn parse_block(&mut self, from: &str) -> Result<Vec<EXPR_Primi>, ZigurateError> {

        let mut content: Vec<EXPR_Primi> = Vec::new();

        if from == "gish" {

            while self.see().is_some() {
                
                let tr_line = self.see();

                match tr_line {
                    Some(virg) => {

                        let virg_val = from_utf8(virg.value)?.chars().next().unwrap();

                        if virg_val == '-' {
                            self.step();
                        } else {
                            break
                        }

                    }
                    None => {
                        break
                    }
                }

                let tk_enums = self.parse_expr()?;
                content.push(tk_enums);
                continue;

            }

            Ok(content)

        } else {
            Ok(content)
        }

    }




    pub fn parse_properties(&mut self, from: &String) -> Result<EXPR_Primi, ZigurateError> {

        // case1: .prop1.prop2.prop3 =
        // case2: .prop1.prop2.prop3)
        // case3: .Enumerator

        let mut props = Vec::new();
        props.push(from.to_string());

        while self.see().is_some() {

            let tok = if let Some(e) = self.see() {
                e
            } else {
                continue
            };

            let val_tok = from_utf8(tok.value)?.chars().next().unwrap();

            match tok.primi {

                Primi::Symbol => {
                    
                    if val_tok == '=' || val_tok == ')' {
                        let expr = EXPR_Primi::PathProperties(props);
                        return Ok(expr)
                    }
                    
                    if val_tok == '.' {
                        self.step();
                        continue;
                    }
                    
                },
                
                Primi::Identifier => {
                    
                    let val_tok = from_utf8(tok.value)?.to_string();

                    props.push(val_tok);

                    if self.i < self.tokens.len() {
                        self.step();

                        let expr = EXPR_Primi::PathProperties(props);
                        return Ok(expr)
                    }

                    continue

                }

                _ => {
                    let expr = EXPR_Primi::PathProperties(props);
                    return Ok(expr)
                }

            }

        }

        let msg = format!("Ocurred a syntax error in `{}`", from);
        Err(ZigurateError::SyntaxError(msg))

    }




    pub fn parse_call_fnc(&mut self) -> Result<Vec<EXPR_Primi>, ZigurateError> {

        let mut args = Vec::new();
        let mut superficial = Vec::new();

        while self.see().is_some() && !self.see().unwrap().value.contains(&b')') {

            match self.parse_expr() {
                Ok(expr) => superficial.push(expr),
                Err(err) => return Err(err)
            }

        }

        self.step(); // eat last ")"

        let size = superficial.len();

        for (i, expr) in superficial.into_iter().enumerate() {
        
            if (i == 0 || i == size) && let EXPR_Primi::Symbol(e) = expr {
                if e == ',' { return Err(ZigurateError::UnexpectedSymbol) }
            } else {

                match expr {
                
                    EXPR_Primi::String(val) => args.push(EXPR_Primi::String(val)),
                    EXPR_Primi::Boolean(val) => args.push(EXPR_Primi::Boolean(val)),
                    EXPR_Primi::Integer(val) => args.push(EXPR_Primi::Integer(val)),
                    EXPR_Primi::Floating(val) => args.push(EXPR_Primi::Floating(val)),
                    
                    EXPR_Primi::Identifier(val) => args.push(EXPR_Primi::Identifier(val)),

                    EXPR_Primi::PathProperties(val) => args.push(EXPR_Primi::PathProperties(val)),
                    
                    _ => todo!()

                }
            }

        }

        Ok(args)

    }




    pub fn parse_expr(&mut self) -> Result<EXPR_Primi, ZigurateError> {

        let tk = self.step();

        match tk.primi {
            

            Primi::String => {

                let value = from_utf8(tk.value)?.to_string();
                let expr = EXPR_Primi::String(value);
                Ok(expr) 

            },
            

            
            Primi::Integer => {

                let value = from_utf8(tk.value).unwrap().parse::<i32>().unwrap();
                let expr = EXPR_Primi::Integer(value);
                Ok(expr) 

            },



            Primi::Floating => {

                let value = from_utf8(tk.value).unwrap().parse::<f32>().unwrap();
                let expr = EXPR_Primi::Floating(value);
                Ok(expr) 

            },



            Primi::Boolean => {

                let value = from_utf8(tk.value).unwrap().parse::<bool>().unwrap();
                let expr = EXPR_Primi::Boolean(value);
                Ok(expr) 

            },



            Primi::Identifier => {

                let value = from_utf8(tk.value)?.to_string();

                let tok_now = self.see();

                match tok_now {

                    Some(tk) => {

                        let tok_now_val = from_utf8(tk.value)?.chars().next().unwrap();

                        match tk.primi {
                            Primi::Symbol => {
                                if tok_now_val == '.' {
                                    let expr = self.parse_properties(&value)?;
                                    Ok(expr)
                                } else {
                                    let expr = EXPR_Primi::Identifier(value);
                                    Ok(expr) 
                                }
                            }
                            _ => {
                                let expr = EXPR_Primi::Identifier(value);
                                Ok(expr) 
                            }
                        }

                    }

                    None => {
                        let expr = EXPR_Primi::Identifier(value);
                        Ok(expr) 
                    }

                }

            },



            Primi::Kwd_Simple => {

                let value = from_utf8(tk.value)?.to_string();
                let expr = EXPR_Primi::KWD_SIMPLE(value);
                Ok(expr) 

            },
            


            Primi::Symbol => {

                let sym_char = from_utf8(tk.value)?.chars().next().unwrap();

                if sym_char == '[' {

                    let data = self.parse_list()?;
                    Ok(data)

                } else {

                    let value = from_utf8(tk.value)?.chars().next().unwrap();
                    let expr = EXPR_Primi::Symbol(value);
                    Ok(expr)

                }

            }
            
            
            _ => Ok(EXPR_Primi::Empty)
        }

    }




    pub fn parse_primary(&mut self) -> Result<STRT_Zilamma, ZigurateError> {

        let node = self.step().to_owned();


        match node.primi {


                Primi::Identifier => {

                    // case1: name = 5
                    // case2: obj.name = 5

                    let tr_cal_name = from_utf8(node.value)?.to_string(); // c1=name | c2=obj

                    let tok_now = self.see().unwrap();

                    let tok_now_val = from_utf8(tok_now.value)?.chars().next().unwrap();

                    // println!("{:?}", tok_now_val);


                    match tok_now.primi {

                        Primi::Symbol => {

                            if tok_now_val == '=' {

                                self.step(); // eat '='
                                let tr_value = self.parse_expr()?;

                                let edit_m_uda = TEMEM_EditVariable { 
                                    name: tr_cal_name, 
                                    signal: SignalType::NewValue,
                                    value: tr_value
                                };

                                let zilamma = STRT_Zilamma::EditUda(edit_m_uda);

                                Ok(zilamma)
                                
                            } else if tok_now_val == '.' {

                                self.step(); // '.'
                                let properties = self.parse_properties(&tr_cal_name)?;

                                let tr_equal_tok = self.see();

                                match tr_equal_tok {
                                    Some(e) => {
                                        
                                        match e.primi {

                                            Primi::Symbol => {

                                                self.step();
                                                let tr_value = self.parse_expr()?;

                                                let edit_m_lunig = TEMEM_EditLunig {
                                                    lunig_id: tr_cal_name,
                                                    path_prop: properties,
                                                    new_value: tr_value
                                                };

                                                let zilamma = STRT_Zilamma::EditLunig(edit_m_lunig);
                                                Ok(zilamma)

                                            }

                                            _ => {
                                                Err(ZigurateError::SyntaxError("SignalType expected. \"=\", \"+=\" or others.".to_string()))
                                            }

                                        }

                                    }
                                    None => {
                                        Err(ZigurateError::SyntaxError("SignalType expected. \"=\", \"+=\" or others.".to_string()))
                                    }
                                }

                            } else {
                                Err(ZigurateError::SyntaxError("SignalType expected. \"=\", \"+=\" or others.".to_string()))
                            }

                        },

                        _ => Err(ZigurateError::SyntaxError("".to_string()))

                    }

                },


                Primi::Kwd_Function => {
                    
                    let fnc_name = from_utf8(node.value)?.to_string();

                    self.step(); // eat "("

                    let fnc_args = self.parse_call_fnc()?;

                    let fnc = TEMEM_Function { name: fnc_name, args: fnc_args };
                    let zilamma = STRT_Zilamma::CallFunction(fnc);

                    return Ok(zilamma)

                },


                Primi::Kwd_Variable => {

                    let ident = self.step();
                    let uda_name = from_utf8(ident.value)?.to_string();

                    let uda_signal = from_utf8(self.step().to_owned().value)?.to_string();

                    let sign_modificator = self.see();

                    match sign_modificator {
                        Some(e) => {

                            match e.primi {

                                Primi::Symbol => {

                                    let v_sign_char = from_utf8(e.value)?.chars().next().unwrap();

                                    if v_sign_char == '@' {

                                        self.step();

                                        match self.see() {

                                            Some(modifier) => {

                                                let modifier_name = from_utf8(modifier.value)?;

                                                match modifier.primi {

                                                    Kwd_Modificators => {

                                                        if modifier_name == "nisa" {

                                                            self.step();

                                                            let uda_value = self.parse_expr()?;

                                                            let uda = TEMEM_TempVariable { 
                                                                name: uda_name, 
                                                                signal: uda_signal, 
                                                                value: uda_value, 
                                                                is_nisa: true 
                                                            };

                                                            let zilamma = STRT_Zilamma::DeclUda(uda);

                                                            return Ok(zilamma)

                                                        } else {
                                                            let msg = format!("You used `@` but `{}` is unknown or not considered modifier.", modifier_name);
                                                            return Err(ZigurateError::SyntaxError(msg))
                                                        }

                                                    }

                                                    _ => {
                                                        let msg = format!("You used `@` but `{}` is unknown or not considered modifier.", modifier_name);
                                                        return Err(ZigurateError::SyntaxError(msg))
                                                    }

                                                }

                                            }
                                            None => {
                                                let msg = format!("You used `@` but the modificator is unknown or not mentioned.");
                                                return Err(ZigurateError::SyntaxError(msg))
                                            }
                                        }


                                    } else {
                                        let msg = format!("For modificator was expected `@`, got wrong");
                                        return Err(ZigurateError::SyntaxError(msg))
                                    }

                                }

                                _ => {

                                    let uda_value = self.parse_expr()?;

                                    let uda = TEMEM_TempVariable { 
                                        name: uda_name, 
                                        signal: uda_signal, 
                                        value: uda_value, 
                                        is_nisa: false 
                                    };

                                    let zilamma = STRT_Zilamma::DeclUda(uda);

                                    return Ok(zilamma)

                                }

                            }

                        } 
                        None => {
                            let msg = format!("Expected a value for `{}`. Got nil.", uda_name);
                            return Err(ZigurateError::SyntaxError(msg))
                        }
                    }

                }



                Primi::kwd_Lunig => {

                    // lunig "Name" as name
                    let tr_lunig_name = self.parse_expr()?;
                    
                    match tr_lunig_name {

                        EXPR_Primi::String(lunig_name) => {

                            let lunig_as = self.parse_expr()?;

                            match lunig_as {
                                EXPR_Primi::KWD_SIMPLE(e) => {

                                    if e == "as" {

                                        let tr_lunig_iden = self.parse_expr()?;

                                        match tr_lunig_iden {

                                            EXPR_Primi::Identifier(iden) => {

                                                let mut lunig = TEMEM_Lunig { id: iden, data: HashMap::new() };
                                                lunig.data.insert("name".to_string(), UdaDeclarationType::String(lunig_name));

                                                let zilamma = STRT_Zilamma::DeclLunig(lunig);

                                                Ok(zilamma)

                                            }

                                            _ => {
                                                let msg = format!("\"Identifier\" expected. Got wrong type value.");
                                                return Err(ZigurateError::SyntaxError(msg))
                                            }

                                        }

                                    } else {
                                        let msg = format!("Expected \"as\" to lunig. Got \"{}\"", e);
                                        return Err(ZigurateError::SyntaxError(msg))
                                    }

                                }
                                _ => {
                                    let msg = format!("'as' expected after \"{:?}\"", lunig_name);
                                    return Err(ZigurateError::SyntaxError(msg))
                                }
                            }

                        } 

                        _ => {
                            let msg = format!("String expected. Got {:?}", tr_lunig_name);
                            return Err(ZigurateError::SyntaxError(msg))
                        }

                    }
                }



                Primi::Kwd_Anki => {

                    // anki.:something()

                    let tr_dot_one = self.step();
                    let tr_dot_one_val = from_utf8(tr_dot_one.value)?.chars().next().unwrap();

                    match tr_dot_one.primi {

                        Primi::Symbol => {

                            if tr_dot_one_val == '.' {

                                let tr_dot_two = self.step();
                                let tr_dot_two_val = from_utf8(tr_dot_two.value)?.chars().next().unwrap();

                                match tr_dot_two.primi {

                                    Primi::Symbol => {

                                        if tr_dot_two_val == ':' {

                                            let fnc_zilamma = self.parse_primary()?;

                                            let anki = TEMEM_Anki { fnc: Box::new(fnc_zilamma) };
                                            let zilamma = STRT_Zilamma::Anki(anki);

                                            Ok(zilamma)

                                        } else {

                                            let msg = format!("Expected `{}` after `anki`. Got `{}`", ".".underline().bold(), tr_dot_one_val);
                                            return Err(ZigurateError::SyntaxError(msg))

                                        }

                                    }

                                    _ => {
                                        let msg = format!("Expected `{}` after `anki`. Got wrong context.", "Symbol".underline().bold());
                                        return Err(ZigurateError::SyntaxError(msg))
                                    }

                                }

                            } else {
                                let msg = format!("Expected `{}` after `anki`. Got `{}`", ".".underline().bold(), tr_dot_one_val);
                                return Err(ZigurateError::SyntaxError(msg))
                            }

                        }

                        _ => {
                            let msg = format!("Expected `{}` after `anki`. Got wrong context.", "Symbol".underline().bold());
                            return Err(ZigurateError::SyntaxError(msg))
                        }

                    }

                }

                

                Primi::Kwd_Maru => {

                    // maru "test"

                    // "test" ak id
                    let tr_path = self.parse_expr()?;

                    match tr_path {
                        EXPR_Primi::String(path) => {

                            let maru = TEMEM_Maru { path: path };
                            let zilamma = STRT_Zilamma::Maru(maru);

                            Ok(zilamma)

                        }
                        _ => {
                            let msg = format!("Expected String as path after `maru`. Got `{:?}`", tr_path);
                            return Err(ZigurateError::SyntaxError(msg))
                        }
                    }

                }



                Primi::Kwd_Gish => {

                    let tr_gish_name = self.parse_expr()?;
                    
                    match tr_gish_name {
                        EXPR_Primi::Identifier(gish_name) => {

                            let tr_two_dots = self.step();
                            let tr_two_dots_val = from_utf8(tr_two_dots.value)?.chars().next().unwrap();

                            match tr_two_dots.primi {

                                Primi::Symbol => {

                                    if tr_two_dots_val == ':' {

                                        let gish_content = self.parse_block("gish")?;

                                        let gish = TEMEM_Gish { id: gish_name, enums: gish_content };
                                        let zilamma = STRT_Zilamma::Gish(gish);

                                        Ok(zilamma)

                                    } else {

                                        let msg = format!("`:` expected after {}.", gish_name);
                                        return Err(ZigurateError::SyntaxError(msg))

                                    }

                                }
                                 
                                _ => {
                                    let msg = format!("`:` expected after {}.", gish_name);
                                    return Err(ZigurateError::SyntaxError(msg))
                                }

                            }

                        }

                        _ => {
                            let msg = format!("Gish identifier expected. Got nil");
                            return Err(ZigurateError::SyntaxError(msg))
                        }
                    }

                }




                _ => todo!()

            }

    }



















    pub fn create(&mut self) -> Result<Vec<STRT_Zilamma>, ZigurateError> {

        let mut tree = Vec::new();

        while self.i < self.tokens.len() {

            let node = self.parse_primary();

            match node {
                Ok(node) => {
                    tree.push(node)
                },
                Err(error) => return Err(error)
            }

        }

        Ok(tree)

    }


}