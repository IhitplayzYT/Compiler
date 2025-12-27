// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused)]
pub mod AST {
    use crate::Lexer_Tok::Lex_Tok::LTOK;
    use std::{cell::{Ref, RefCell}, rc::Rc};
    pub type link<T> = Rc<RefCell<T>>;
    
    pub enum Val{
        Int(i64),
        Float(f64),
        String(String),
    }


    #[derive(Clone,Copy)]
    pub enum Expr<'a>{
        Int(i64),
        Float(&'a str),
        String(&'a str),
        Null,
        Ident(&'a str),
        Binary_op {
            op: link<Expr>, // Represents the Node for an operator
            left: link<Expr>,  // Unary ops such as shorthands can be represented by considering the left pointer ab the variable on which the unary op being done
            right: link<Expr>
        },
        Unary_op{
            op:LTOK,
            operand: link<Expr>,
        }
    }

    pub enum Statmnt {
        Expression(Expr),
        Let {
            name: String, // Deals with let,mut and const
            mutable: bool,
            value: link<Expr>,
        },
        If {
            cond: link<Expr>,
            then_branch: Vec<link<Statmnt>>, // Deals with if and else
            else_branch: Option<Vec<link<Statmnt>>>,
        },
        While {
            cond: link<Expr>, // While loop
            body: Vec<link<Statmnt>>,
        },
        For {
            init: Vec<link<Expr>>,
            cond: Option<link<Expr>>, // For loop SYNTAX : C and Rust hybrid type
            step: Option<link<Expr>>,
            body: Vec<link<Statmnt>>,
        },
        None
    }
    pub enum Code{
        Expr(Expr),
        Statmnt(Statmnt),
    }
    pub struct AST_Node{
        pub code : Code,
        pub children : Vec<Option<link<AST_Node>>>,
    }
    impl AST_Node{
    pub fn new(c : Code) -> Self{
       Self {code:c,children:vec![]} 
    }
pub fn as_expr<'a>(& 'a self) -> & 'a Expr{
    if let Code::Expr(e) = &self.code{
    return e;
    }
    else{
    return &Expr::Null; 
    }
}

    pub fn as_statmnt<'a>(&'a self) ->  &'a Statmnt{
        if let Code::Statmnt(e) = &self.code{
        return e;
        }
        else{
        return &Statmnt::None;
        }

    }
}


}
