// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

// :TODO
//VARIB(String),  // -> (Variable)
//        INT(i64),       // -> (Integer Literal)
//        FLOAT(f64),     // -> (Floating point Literal)
//        STRING(String), // -> (String Literal)
//        BOOL(bool),     // -> (Boolean Literal)
//        CHAR(char),     // -> (Character Literal)
//
//
#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused)]
pub mod Tokens {

    pub enum Keyword {
        LET,
        IF,
        ELSE,
        CONST,
        MUT,
        FN,
        LOOP,
        WHILE,
        FOR,
    }

    pub enum Arithemetic {
        ADDITION,
        SUBTRACTION,
        MULTIPLICATION,
        DIVISION,
        MODULO,
        LSHIFT,
        RSHIFT,
        AND,
        OR,
        NOT,
        XOR,
        ASSGN,
    }

    pub enum Logical {
        AND,
        OR,
        NOT,
    }

    pub enum Comparison {
        LT,
        GT,
        EQ,
        N_EQ,
        LT_EQ,
        GT_EQ,
    }

    pub enum Token {
        Keyword,
        Variable(String),
        Arithemetic,
        Logical,
        Comparison,
        Literal,
        SEMICOLON,
        EOF,
    }
}
