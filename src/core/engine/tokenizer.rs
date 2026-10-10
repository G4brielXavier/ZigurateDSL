use crate::core::engine::eyes::Eyes;


use crate::core::utils::primi::Primi;
use crate::core::utils::table::Table;
use crate::core::utils::token::Token;





pub struct Tokenizer<'a> {
    pub eyes: Eyes<'a>,
    pub zigu_table: Table
}


impl<'a> Tokenizer<'a> {

    pub fn open(src: &'a [u8]) -> Self {
        Self {
            eyes: Eyes::open(src),
            zigu_table: Table::get()
        }
    }

    pub fn ignite(&mut self) -> Vec<Token<'a>> {

        let mut tokens: Vec<Token> = Vec::new();

        while !self.eyes.look().is_none() {

            let char = self.eyes.look();

            if let Some(char) = char {

                if char.is_ascii_whitespace() {
                    self.eyes.blink();
                    continue;
                }

                if char == b'/' && self.eyes.look_next().unwrap() == b'/' {
                    self.see_comments();
                    continue;
                }

                if char.is_ascii_alphabetic() || char == b'_' {
                    tokens.push(self.see_identifier());
                    continue;
                }

                if char == b'\"' {
                    tokens.push(self.see_string());
                    continue;
                }

                if char.is_ascii_digit() {
                    tokens.push(self.see_numeric());
                    continue;
                }

                if self.zigu_table.symb.contains(&char) {
                    tokens.push(self.see_symbol());
                    continue;
                }

            }

        }

        tokens
    
    }


    fn see_numeric(&mut self) -> Token<'a> {

        let st_ln = self.eyes.ln;
        let st_col = self.eyes.col;

        let i_idx = self.eyes.i;

        while self.eyes.look().is_some() && (self.eyes.look().unwrap().is_ascii_digit() || self.eyes.look().unwrap() == b'.') {
            self.eyes.blink();
        }

        let value = &self.eyes.src[i_idx..self.eyes.i];
        let has_dot = b'.';

        let primi = if value.contains(&has_dot) {
            Primi::Floating
        } else {
            Primi::Integer
        };

        Token {

            primi: primi,
            value: value,

            st_ln,
            st_col,

            end_ln: self.eyes.ln,
            end_col: self.eyes.col

        }

    }

    fn see_string(&mut self) -> Token<'a> {

        let st_ln = self.eyes.ln;
        let st_col = self.eyes.col;

        self.eyes.blink();

        let i_idx = self.eyes.i;

        // "text"

        while self.eyes.look().is_some() && self.eyes.look().unwrap() != b'\"' {
            self.eyes.blink();
        }

        let val = &self.eyes.src[i_idx..self.eyes.i];
        self.eyes.blink();


        let primi = Primi::String;

        return Token {

            primi: primi,
            value: val,

            st_ln,
            st_col,

            end_ln: self.eyes.ln,
            end_col: self.eyes.col

        }

    }

    fn see_symbol(&mut self) -> Token<'a> {

        let st_ln = self.eyes.ln;
        let st_col = self.eyes.col;

        let i_idx = self.eyes.i;

        self.eyes.blink();

        let char = &self.eyes.src[i_idx..self.eyes.i];            

        let primi = Primi::Symbol;

        Token {

            primi: primi,
            value: char,

            st_ln,
            st_col,

            end_ln: self.eyes.ln,
            end_col: self.eyes.col

        }

    }

    fn see_identifier(&mut self) -> Token<'a> {

        // my_name

        let st_ln = self.eyes.ln;
        let st_col = self.eyes.col;

        let i_idx = self.eyes.i;

        while self.eyes.look().is_some() && (self.eyes.look().unwrap().is_ascii_alphanumeric() || self.eyes.look().unwrap() == b'_') {
            self.eyes.blink();
        }

        let b_val = &self.eyes.src[i_idx..self.eyes.i];

        let primi = 
            if self.zigu_table.kwd_functions.contains(&b_val) {
                Primi::Kwd_Function
            } else if self.zigu_table.kwd_variable.contains(&b_val) {
                Primi::Kwd_Variable
            } else if self.zigu_table.kwd_modificators.contains(&b_val) {
                Primi::Kwd_Modificators
            } else if self.zigu_table.kwd_simple.contains(&b_val) {
                Primi::Kwd_Simple
            } else if self.zigu_table.kwd_lunig == b_val {
                Primi::kwd_Lunig
            } else if self.zigu_table.kwd_special.contains(&b_val) {
                Primi::Boolean
            } else if self.zigu_table.kwd_anki == b_val {
                Primi::Kwd_Anki
            } else if self.zigu_table.kwd_maru == b_val {
                Primi::Kwd_Maru
            } else if self.zigu_table.kwd_gish == b_val {
                Primi::Kwd_Gish
            } else {
                Primi::Identifier
            };

        Token {
            primi,
            value: b_val,

            st_ln,
            st_col,

            end_ln: self.eyes.ln,
            end_col: self.eyes.col
        }


    }

    fn see_comments(&mut self) {

        self.eyes.blink(); 
        self.eyes.blink();

        while self.eyes.look().unwrap() != b'\n' {
            self.eyes.blink();
        }

    }


}