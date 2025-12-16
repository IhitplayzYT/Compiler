// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

mod Ast;
mod Frontend;
mod Helper;
mod Lexer_Tok;
mod Parser;
mod Parser_Tok;
mod Semantic_Analysis;
mod Tokeniser;
use std::env;

use crate::Helper::utilities::read_file;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        std::process::exit(0);
    }
    let file =
        Helper::utilities::read_file(&arguments[1]).unwrap_or_else(|e| std::process::exit(0));

    let mut LEXER = Tokeniser::Tokeniser::Lexer::new(file);
    LEXER.Tokenise();
    println!("{:?}", LEXER.Lexer_Output);
}
