// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused)]
pub mod PARSER {
    use std::{collections::HashMap, hash::Hash};
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::Parser_Tok::Tokens::Token;
    use crate::{Ast::{self, AST::{self,Code,Val,Type,BIN_OP,UN_OP,Declare,link,Statmnt,Expr}},Lexer_Tok::Lex_Tok::LTOK,Ident_table::Ident};
    use crate::Errors::Err::{ParserError,Parser_ret};
    pub struct Parser {
        input: Vec<LTOK>,
        idx:usize,
    }
    impl Parser{
        // Constructor
        fn new(v:Vec<LTOK>) -> Self{
            Self{input:v,idx:0}
        }
        // Helper
        fn next(&mut self) -> LTOK{
            if self.input.is_empty() || self.idx == self.input.len() {
                return LTOK::EOF;
            }
            self.idx += 1;
            self.peek().clone()
        }
        fn peek(&self) -> &LTOK{
            if self.input.is_empty() || self.idx == self.input.len() {
                return &LTOK::EOF;
            }
            self.input.get(self.idx).unwrap_or(&LTOK::EOF)
        }

        fn check(&mut self,e: &LTOK) -> bool{
           std::mem::discriminant(self.peek()) == std::mem::discriminant(e)
        }

        fn consume(&mut self,e: &LTOK) -> Parser_ret<LTOK>{
         if self.check(e) {
            return Ok(self.next());
         }
         Err(ParserError::UnexpectedToken { expected:format!("{:?}",e.clone()), got:format!("{:?}",self.peek().clone()) })
        }

        fn eval_declare(&mut self) -> bool{
            match self.peek(){
            LTOK::FN => self.eval_fxn(),
                _ => {return false;}
            };
            true
        } 

        fn eval_fxn(&mut self) -> Parser_ret<&LTOK>{
        self.consume(&LTOK::FN)?;
        let name = match self.next(){
            LTOK::IDENT(x) => x,
            _ => {return Err(ParserError::UnexpectedToken{expected:"Fxn name\n".to_string(),got:"INVALID_TOK\n".to_string()});}
        };
        self.consume(&LTOK::LPAREN)?;
        let param = self.eval_params()?;
        self.consume(&LTOK::RPAREN)?;
        self.consume(&LTOK::ARROW)?;
        let rtype = self.eval_type()?;
        self.consume(&LTOK::LBRACE)?;
        let body = self.eval_block()?;
        self.consume(&LTOK::RBRACE)?;
        self.consume(&LTOK::SEMICOLON)?;
        return Ok(&LTOK::EOF);
        }

        fn eval_params(&mut self) -> Parser_ret<&LTOK>{
            Ok(&LTOK::SPECIAL_TOK)
        }
        fn eval_type(&mut self) -> Parser_ret<&LTOK>{
            Ok(&LTOK::SPECIAL_TOK)
        }
        fn eval_block(&mut self) -> Parser_ret<&LTOK>{
            Ok(&LTOK::SPECIAL_TOK)
        }


//        fn eval_let(&mut self) -> Option<Statmnt>{
//        }
//
//        fn eval_if_else(&mut self) -> Option<AST_Node>{
//        }
//        
//        fn eval_while(&mut self) -> Option<AST_Node>{
//        }
        
        // Extremely simple that deals with runtime panics on is own since the eval_expr also deals with the untme panics
//        fn eval_for(&mut self) -> Option<AST_Node>{
//        }
//
//       fn eval_expr(&mut self)->Option<AST_Node>{
//        }

        fn binding_pow(tok : &LTOK) -> (i32,i32){ 
            match *tok {
                LTOK::LPAREN => (0,100),
                LTOK::RPAREN => (100,0),
                LTOK::BANG | LTOK::TILDA | LTOK::MODULO  => (0,99),
                LTOK::DIV | LTOK::STAR => (60,61),
                LTOK::PLUS | LTOK::MINUS => (50,51),
                LTOK::LSHIFT | LTOK::RSHIFT => (45,46),
                LTOK::AMP => (40, 41),
                LTOK::CARET => (39, 40),
                LTOK::PIPE => (38, 39),
                LTOK::EQ | LTOK::N_EQ | LTOK::GT | LTOK::GT_EQ | LTOK::LT_EQ | LTOK::LT => (30, 31),
                LTOK::ANDAND => (20, 21),
                LTOK::OROR => (10, 11),
                LTOK::S_AMP | LTOK::S_PIPE | LTOK::S_CARET |LTOK::S_PLUS |LTOK::S_MULT |LTOK::S_MINUS |LTOK::S_DIV |LTOK::S_MOD | LTOK::ASSGN => (0,1),
                _ => (-1,-1),
            }

        }

        fn Parse(&mut self) -> bool {          
            if self.input.is_empty(){return false;}
            
        



            true
        }


    }

}
