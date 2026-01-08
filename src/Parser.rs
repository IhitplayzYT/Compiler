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
// HELPER
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
        fn match_token(&mut self,token:&[LTOK]) -> bool{
            for i in token{
                if self.check(i){
                    self.next();
                    return true;
                }
            }
            false
        }
        fn consume(&mut self,e: &LTOK) -> Parser_ret<LTOK>{
         if self.check(e) {
            return Ok(self.next());
         }
         Err(ParserError::UnexpectedToken { expected:format!("{:?}",e.clone()), got:format!("{:?}",self.peek().clone()) })
        }
//HELPER

        fn eval_declare(&mut self) -> Parser_ret<Declare>{
            match self.peek(){
            LTOK::FN => self.eval_fxn(),
                _ => {return Err(ParserError::Invalid_Code);}
            }

        } 
        
        fn eval_fxn(&mut self) -> Parser_ret<Declare>{
        self.consume(&LTOK::FN)?;
        let name = match self.next(){
            LTOK::IDENT(x) => x,
            _ => {return Err(ParserError::UnexpectedToken{expected:"Fxn name\n".to_string(),got:"INVALID_TOK\n".to_string()});}
        };
        self.consume(&LTOK::LPAREN)?;
        let param = self.eval_params()?;
        self.consume(&LTOK::RPAREN)?;

        let rtype = if self.match_token(&[LTOK::ARROW]){
            Some(self.eval_type()?)
        }else{
            None
        };

        let body = self.eval_block()?;
        self.consume(&LTOK::RBRACE)?;
        return Ok(Declare::Function { name:name, rtype: rtype, args: param, body: body });
        }

        fn eval_params(&mut self) -> Parser_ret<Vec<(String,Type)>>{
        let mut ret:Vec<(String,Type)> = Vec::new();

        if !self.check(&LTOK::RPAREN) {
            loop{
                let name = match self.next() {
                    LTOK::IDENT(x) => x,
                    t => {return Err(ParserError::UnexpectedToken { expected: "Parameter".to_string(), got: format!("{:?}", t) })}
                };
                self.consume(&LTOK::COLON)?;
                let type_varib = self.eval_type()?;
                ret.push((name,type_varib));
                if !self.match_token(&[LTOK::COMMA]){break;}
            }
        }
        Ok(ret)
        }

        fn eval_type(&mut self) -> Parser_ret<Type>{
            match self.next(){
                LTOK::INT_TYPE => Ok(Type::INT),
                LTOK::FLOAT_TYPE => Ok(Type::FLOAT),
                LTOK::STRING_TYPE => Ok(Type::STRING),
                z => Err(ParserError::UnexpectedToken { expected: "type".to_string(), got:  format!("{:?}",z)})
            }
        }

        fn eval_block(&mut self) -> Parser_ret<&LTOK>{
            Ok(&LTOK::SPECIAL_TOK)
        }
        fn eval_expr(&mut self) -> Parser_ret<&LTOK>{
            Ok(&LTOK::SPECIAL_TOK)
        }


        fn eval_if_else(&mut self) -> Parser_ret<&LTOK>{
            self.consume(&LTOK::IF)?;
            self.consume(&LTOK::LPAREN)?;
            let cond = self.eval_expr()?;
            self.consume(&LTOK::RPAREN)?;
            self.consume(&LTOK::LBRACE)?;
            let if_block = self.eval_block()?;
            self.consume(&LTOK::RBRACE)?;

            let else_block = {
            if let &LTOK::ELSE = self.peek(){
                self.consume(&LTOK::LBRACE)?;
                let else_block = self.eval_block()?;
                self.consume(&LTOK::RBRACE)?;
                else_block
            }
            else{
                None
            }
        };
          // (xx)

            Ok(&LTOK::SPECIAL_TOK)
        }

        fn eval_let(&mut self) ->   Parser_ret<&LTOK>{
            self.consume(&LTOK::LET)?;
            let mut mut_flag = 0 ;
            let mut varib_name = "".to_string();
            match self.peek() {
                &LTOK::MUT => {mut_flag= 1;
                    varib_name = if let &LTOK::IDENT(x) = self.consume(&LTOK::)
                
                
                }
                &LTOK::IDENT(x) => {varib_name = x;
                
                
                
                
                }
                _ => {return Err(ParserError::UnexpectedToken { expected: "mut | <var>".to_string(), got:format!("{:?}",self.peek())});}
            }


            Ok(&LTOK::SPECIAL_TOK)
        }
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

        fn Parse(&mut self) -> Parser_ret<Code> {         
            let mut ret =  Vec::new();
            while !self.check(&LTOK::EOF){
                ret.push(self.eval_declare()?);
            }
            Ok(Code { Program: ret })
        }


    }

}
