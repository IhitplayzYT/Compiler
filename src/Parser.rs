// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals)]
pub mod PARSER {
    use crate::{Ast::{AST::{Code,Type,Declare,Statmnt,Expr}},Lexer_Tok::Lex_Tok::LTOK};
    use crate::Errors::Err::{ParserError,Parser_ret};
    pub struct Parser {
        input: Vec<LTOK>,
        idx:usize,
    }
    impl Parser{
        /* ******************************** CONSTRUCTOR ********************************  */
        fn new(v:Vec<LTOK>) -> Self{
            Self{input:v,idx:0}
        }
        /* ******************************** CONSTRUCTOR ********************************  */


        /* ******************************** MAIN ********************************  */
        fn Parse(&mut self) -> Parser_ret<Code> {         
            let mut ret =  Vec::new();
            while !self.check(&LTOK::EOF){
                ret.push(self.eval_declare()?);
            }
            Ok(Code { Program: ret })
        }
        /* ******************************** MAIN ********************************  */


        /* ******************************** HELPER ********************************  */
        fn next(&mut self) -> LTOK{
            if self.input.is_empty() || self.idx == self.input.len() {
                return LTOK::EOF;
            }
            self.idx += 1;
            self.peek().clone()}


        fn peek(&self) -> &LTOK{
            if self.input.is_empty() || self.idx == self.input.len() {
                return &LTOK::EOF;
            }
            self.input.get(self.idx).unwrap_or(&LTOK::EOF)}

        fn check(&mut self,e: &LTOK) -> bool{std::mem::discriminant(self.peek()) == std::mem::discriminant(e)}


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

        /* ******************************** HELPER ********************************  */

        
        /* ******************************** FUNCTIONS ********************************  */
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
            tok => {return Err(ParserError::UnexpectedToken{expected:"Fxn name".to_string(),got:format!("{:?}",tok)});}};

        self.consume(&LTOK::LPAREN)?;

        let param = self.eval_params()?;

        self.consume(&LTOK::RPAREN)?;

        let rtype = 
            if self.match_token(&[LTOK::ARROW]) {Some(self.eval_type().unwrap())}
            else{None};

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
        /* ******************************** FUNCTIONS ********************************  */


        /* ******************************** LET & CONST ********************************  */

        fn eval_let(&mut self) -> Parser_ret<Statmnt> {
        self.consume(&LTOK::LET)?;
        let mutable = self.match_token(&[LTOK::MUT]);
        let name: String = match self.next() {
            LTOK::IDENT(x) => x,
            t => return Err(ParserError::UnexpectedToken { expected: "VARIB_NAME".to_string(), got:  format!("{:?}",t)})
        };
        let annot = if self.match_token(&[LTOK::COLON]) {
            Some(self.eval_type()?)
        }
        else{
            None
        };

        self.consume(&LTOK::ASSGN)?;
        let val = self.eval_expr()?;
        self.consume(&LTOK::SEMICOLON)?;
        Ok(Statmnt::Let { name, mutable, type_annot:annot, value: val })
        }

        fn eval_const(&mut self) -> Parser_ret<Statmnt> {
            self.consume(&LTOK::CONST)?;
            let name: String = match self.next() {
            LTOK::IDENT(x) => x,
            t => return Err(ParserError::UnexpectedToken { expected: "VARIB_NAME".to_string(), got:  format!("{:?}",t)})
            };
            let annot = if self.match_token(&[LTOK::COLON]) {
                Some(self.eval_type()?)
            }
            else{
                None
            };
            self.consume(&LTOK::ASSGN)?;
            let val = self.eval_expr()?;
            self.consume(&LTOK::SEMICOLON)?;
            Ok(Statmnt::Let { name, mutable:true, type_annot:annot, value: val })
        }
   
        /* ******************************** LET & CONST ********************************  */

        /* ******************************** IF-ELSE ********************************  */

        fn eval_if_else(&mut self) -> Parser_ret<Statmnt>{
            self.consume(&LTOK::IF)?;
           
            let cond = self.eval_expr()?;
            self.consume(&LTOK::LBRACE)?;
            let then_branch = self.eval_block()?;
            self.consume(&LTOK::RBRACE)?;


            let else_branch = if self.match_token(&[LTOK::ELSE]) {
                if self.check(&LTOK::IF){
                    Some(vec![self.eval_if_else()?])
                }else{
                    self.consume(&LTOK::LBRACE)?;
                   let bl = self.eval_block()?;
                   self.consume(&LTOK::RBRACE)?;
                   Some(bl)
                }
                }else{
                    None
                };
                       
            Ok(Statmnt::If { cond , then_branch, else_branch })
        }

        /* ******************************** IF-ELSE ********************************  */



        /* ******************************** FOR-WHILE-LOOP ********************************  */

        fn eval_for(&mut self) -> Parser_ret<Statmnt>{
            self.consume(&LTOK::FOR)?;
            let var_name = match self.next() {
                LTOK::IDENT(x) => x,
                _ => return Err(ParserError::Invalid_Code),
            };
            
            let lb = self.eval_expr()?;
            let rb = self.eval_expr()?;
            self.consume(&LTOK::LBRACE)?;
            let body = self.eval_block()?;
            self.consume(&LTOK::RBRACE)?;   
            Ok(Statmnt::For { var_name, lb, rb, body })
        }

        fn eval_while(&mut self) -> Parser_ret<Statmnt> {
            self.consume(&LTOK::WHILE)?;
            let cond = self.eval_expr()?;
            self.consume(&LTOK::LBRACE)?;
            let body = self.eval_block()?;
            self.consume(&LTOK::RBRACE)?;
            Ok(Statmnt::While { cond, body })
        }

        fn eval_loop(&mut self) -> Parser_ret<Statmnt>{
        self.consume(&LTOK::LOOP)?;
        self.consume(&LTOK::LBRACE)?;
        let body = self.eval_block()?;
        self.consume(&LTOK::RBRACE)?;    
        Ok(Statmnt::Loop { body })
        }

        
        /* ******************************** FOR-WHILE-LOOP ********************************  */



        /* ******************************** HELPER ********************************  */
        
        fn eval_type(&mut self) -> Parser_ret<Type>{
            match self.next(){
                LTOK::INT_TYPE => Ok(Type::INT),
                LTOK::FLOAT_TYPE => Ok(Type::FLOAT),
                LTOK::STRING_TYPE => Ok(Type::STRING),
                z => Err(ParserError::UnexpectedToken { expected: "INT|FLOAT|STRING".to_string(), got:  format!("{:?}",z)})
            }
        }

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

        fn eval_return(&mut self) -> Parser_ret<Statmnt> {
            self.consume(&LTOK::RETURN)?;
            let val = match self.next() {
                LTOK::SEMICOLON => None,
                _ => Some(self.eval_expr()?),
            };

            Ok(Statmnt::Return(val))
        }

        fn eval_tuple_types(&mut self) -> Parser_ret<Vec<Type>>{
            let mut ret = Vec::new();
            while !self.check(&LTOK::LPAREN){
                ret.push(self.eval_type()?);
                if !self.match_token(&[LTOK::COMMA]){
                    break;
                }
            }
            Ok(ret)
        }

        /* ******************************** HELPER ********************************  */


    
    /* ******************************** BLOCKS & STATEMENTS ********************************  */

        fn eval_statmnt(&mut self) -> Parser_ret<Statmnt>{
            match self.peek() {
                LTOK::LET => {self.eval_let()},
                LTOK::CONST => {self.eval_const()},
                LTOK::IF  => {self.eval_if_else()},
                LTOK::WHILE => {self.eval_while()},
                LTOK::FOR => {self.eval_for()},
                
                LTOK::BREAK => {
                self.next();
                self.consume(&LTOK::SEMICOLON);
                Ok(Statmnt::Break)
                },

                LTOK::CONTINUE =>{
                self.next();
                self.consume(&LTOK::SEMICOLON);
                Ok(Statmnt::Continue)
                },

                LTOK::RETURN => self.eval_return(),
                LTOK::LBRACE => {
                    self.next();
                    let blk = self.eval_block()?;
                    self.consume(&LTOK::RBRACE)?;
                    Ok(Statmnt::Block(blk))
                },
                _ => self.eval_reassign(),

            }  
        }

        fn eval_block(&mut self) -> Parser_ret<Vec<Statmnt>>{
            let mut statmnts:Vec<Statmnt> = Vec::new();
            while !self.check(&LTOK::RBRACE) && !self.check(&LTOK::EOF){
                statmnts.push(self.eval_statmnt()?);
            }
            Ok(statmnts)
        }

    /* ******************************** BLOCKS & STATEMENTS ********************************  */

    /* ******************************** EXPRESSIONS ********************************  */


        fn eval_expr(&mut self) -> Parser_ret<Expr>{
            
        }

        fn eval_reassign(&mut self) - > Parser_ret<Expr> {


        }
        
        
           
   
    /* ******************************** EXPRESSIONS ********************************  */

    }

}
