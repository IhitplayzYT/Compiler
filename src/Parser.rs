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
    use crate::{Ast::{self, AST::{self,AST_Node,link,Statmnt,Expr,Code}}, Lexer_Tok::Lex_Tok::LTOK};

    pub struct Parser {
        input: Vec<LTOK>,
        idx:usize,
        ast: Option<AST::AST_Node>,
    }
    impl Parser{
        fn new(v:Vec<LTOK>) -> Self{
            Self{ast:None,input:v,idx:0}
        }
        fn next(&mut self) -> &LTOK{
            if self.input.is_empty() {
                return &LTOK::EOF;
            }
            self.idx += 1;
            self.input.get(self.idx - 1).unwrap_or(&LTOK::EOF)
        }
        fn peek(&self) -> &LTOK{
            if self.input.is_empty() {
                return &LTOK::EOF;
            }
            self.input.get(self.idx).unwrap_or(&LTOK::EOF)
        }
        fn expect(&mut self,e: LTOK) -> bool{
            if &e == self.next(){
                return true;
            }
            false
        }

        fn eval_let(&mut self) -> Option<AST_Node>{
            //let mut ret:Option<AST_Node> = None;
            //match self.peek().clone(){
            //    LTOK::MUT =>{ 

            //    },
            //    LTOK::IDENT(x) => {},
            //    _=>  return None,
            //}
            return Some(AST_Node { 
                code: Code::Statmnt(Statmnt::Let { name: (), mutable: (), value: () }),
                children: (None,None),
            });
        }

        fn eval_if_else(&mut self) -> Option<AST_Node>{
            let mut ret:Option<AST_Node> = None;
            return Some(AST_Node { 
                code: Code::Statmnt(Statmnt::If { cond: (), then_branch: (), else_branch: () }),
                children: (None,None),
            });
        }
        
        fn eval_while(&mut self) -> Option<AST_Node>{
            return Some(AST_Node { 
                code: Code::Statmnt(Statmnt::While { cond: (), body: ()}),
                children: (None,None),
                });
        }

        fn eval_for(&mut self) -> Option<AST_Node>{
            return Some(AST_Node{
                 code: Code::Statmnt(Statmnt::For { init: self.eval_expr(), cond: self.eval_expr(), step: self.eval_expr(), body: self.Parse() }),children:(None,None),
                 });
        }

       fn eval_expr(&mut self)->Option<AST_Node>{
          return Some(AST_Node{
               code: Code::Expr(),children:(None,None),
        });
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

        fn Parse(&mut self) -> bool {          
            if self.input.is_empty(){return false;}
            let mut ret: Option<AST_Node> = None;
            
        



            self.ast = ret;
            true
        }


    }

}
