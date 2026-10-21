// SPDX-License-Identifier: GPL-3.0-only
//
// Copyright (C) 2025 Ihit Acharya
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

//   Compiler.rs   //
// Contains the main Compiler API

pub mod compiler{
use crate::{Frontend::Frontend::Frontend, Helper::Main::CLI};
use crate::Errors::Err::{CompilerError, CompilerReturn, ERROR};
    /// Compiler structure[MAIN API]
    /// 
    /// # Traits
    /// - Derived<br/>
    ///     - Debug
    ///     - Clone
    /// # Example
    /// ```
    /// Compiler{
    ///     frontend: Frontend,
    /// }
    /// ```    
    #[derive(Debug,Clone)]
    pub struct Compiler{
        fname: Option<String>,
        rules_engine: CLI,
        frontend:Frontend,
    }


    impl Compiler{
        /// Create and Initialise a new Compiler struct
        /// 
        /// # Arguments
        /// file_path : &str  -> Path to the file being compiled
        /// 
        /// # Example
        /// ```
        /// Compiler::new("myfile.c");
        /// ```    
        

        pub fn new() -> Self{
            Self{fname:None,rules_engine:CLI::new(),frontend:Frontend::new()}
        }

        pub fn Compile(&mut self) -> CompilerReturn<bool>{
            self.rules_engine.parse_clargs().map_err(|x| {CompilerError::RULES_ERROR(format!("{x:?}"))})?;
            let k = self.frontend.Exec(self.rules_engine.clone()).map_err(|z|{
                match z{
                    ERROR::Semerr(x) => {CompilerError::FRONTEND_ERROR(format!("{x:?}"))},
                    ERROR::Parseerr(x) => {CompilerError::FRONTEND_ERROR(format!("{x:?}"))},
                    ERROR::Irgenerr(x) => {CompilerError::FRONTEND_ERROR(format!("{x:?}"))},
                    _ => panic!("Not possible")
                }
            })?;

            Ok(true)
        }

        pub fn Link(&mut self) -> CompilerReturn<bool>{
       
            Ok(true)
        }

        pub fn Run(&mut self) -> CompilerReturn<bool>{
       
            Ok(true)
        }         

    }

}