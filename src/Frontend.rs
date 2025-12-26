// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused)]

pub mod Frontend {
    use crate::Parser::PARSER::{self, Parser};
    use crate::Semantic_Analysis::Analyser::Semantilizer;
    use crate::Tokeniser::Tokeniser::Lexer;
    pub struct Frontend {
        parser: Parser,
        lexer: Lexer,
        Semantics: Semantilizer,
    }
}
