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
    
    #[derive(Clone,PartialEq,Debug)]
    pub enum Val{
        Int(i64),
        Float(f64),
        String(String),
        Null
    }

    #[derive(Clone,PartialEq,Debug)]
    pub enum Type{
        INT,
        FLOAT,
        STRING,
        NULL,
    } 

    #[derive(Clone,PartialEq,Debug)]
    pub enum BIN_OP{
        Add,Sub,Mul,Div,Mod,Eq,Lt_eq,Gt_eq,Lt,Gt,N_eq,Amp,Pipe,Xor,Lshift,Rshift,And,Or
    }

    #[derive(Clone,PartialEq,Debug)]
    pub enum UN_OP{
        Tilda,Bang,Neg
    }

    #[derive(Clone,Debug,PartialEq)]
    pub enum Expr{
        Int(i64),
        Float(f64),
        String(String),
        Null,
        Ident(String),

        Binary_op {
            op: BIN_OP, // Represents the Node for an operator
            left: link<Expr>,  // Unary ops such as shorthands can be represented by considering the left pointer ab the variable on which the unary op being done
            right: link<Expr>
        },

        Unary_op{
            op:UN_OP,
            operand: link<Expr>,
        },

        Fxn_call{
            name: String,
            args: Vec<Expr>,
        },

    }
    #[derive(Clone,PartialEq,Debug)]
    pub enum Declare{
        Function { 
            name:String,
            rtype: Type,
            args : Vec<(String,Type)>,
            body : Vec<Statmnt>,
        },
    }




    #[derive(Clone,PartialEq,Debug)]
    pub enum Statmnt {
        Let {
            name: String, // Deals with let,mut and const
            mutable: bool,
            type_annot: Type,
            value: link<Expr>,
        },
        If {
            cond: Expr,
            then_branch: Vec<Statmnt>, // Deals with if and else
            else_branch: Option<Vec<Statmnt>>,
        },
        Assignment{
            name:String,
            val: Expr,
        },
        While {
            cond: Expr, // While loop
            body: Vec<Statmnt>,
        },
        For_C {
            init: Vec<Expr>,
            cond: Option<Expr>, // For loop SYNTAX : C
            step: Option<Expr>,
            body: Vec<Statmnt>,
        },
        For_Rust{
            var_name: String, // For loop SYNTAX : Rust
            lb: Expr,
            rb: Expr,
        },
        Expr(Expr),
        Break,
        Continue,
        Return(Option<Expr>),
        Block(Vec<Statmnt>)

    }
    pub struct Code{
        pub Program: Vec<Declare>,
    }

}
