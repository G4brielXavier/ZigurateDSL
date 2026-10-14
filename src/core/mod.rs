pub mod error;
pub mod engine;
pub mod utils;


use std::println;

use crate::core::engine::tokenizer::Tokenizer;
use crate::core::engine::ast::AST;
use crate::core::error::ZigurateError;

use crate::enki_vm::enki::EnkiVM;
use crate::enki_vm::{self, EnkiInstruction};
use crate::enki_vm::compiler::enki_compiler;








pub fn zigurage_compiler_simple(byte_content: &[u8]) -> Result<Vec<EnkiInstruction>, ZigurateError> {

    let mut tokenizer = Tokenizer::open(byte_content);
    let tokens = tokenizer.ignite();

    // for i in tokens.iter() {
    //     println!("{:?}: {}", i.primi, from_utf8(i.value).unwrap());
    // }

    let mut ast = AST::open(tokens);
    let ast = ast.create()?;

    // println!("{:?}", ast);
    
    let sig_enki_byte = enki_compiler(ast)?;

    // println!("{:?}", sig_enki_byte);
    
    Ok(sig_enki_byte)

}

pub fn zigurate_compiler_enki(enki_vm: &mut EnkiVM) -> Result<(), ZigurateError> {

    println!();
    enki_vm.run()?;

    // println!("{:?}", enki_vm.instructions);

    Ok(())

}
