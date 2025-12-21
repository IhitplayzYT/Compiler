pub mod PARSER {
    use std::{collections::HashMap, hash::Hash};
    use once_cell::sync::Lazy;
    use crate::{Ast::{self, AST::{self, link}}, Lexer_Tok::Lex_Tok::LTOK};
    pub static Presced : Lazy<HashMap<LTOK,(f32,f32)>> = Lazy::new(|| {HashMap::from([
        (LTOK::PLUS,(2.0,1.9)),(LTOK::DIV,(3.2,3.1)),(LTOK::MINUS,(2.2,2.1)),(LTOK::STAR,(3.0,2.9)),(LTOK::BANG,(-1.0,2.8)),(LTOK::LPAREN,(0.0,10.0)),(LTOK::RPAREN,(10.0,0.0)),(LTOK::AMP,(1.2,1.15)),(LTOK::PIPE,(1.1,1.0)),(LTOK::CARET,(1.3,1.25)),(LTOK::ANDAND,(0.2,0.15)),(LTOK::OROR,(0.1,0.05)),(LTOK::TILDA,(0.0,1.4)),(LTOK::ASSGN,(0.0,0.0)),(LTOK::EQ,(0.95,0.9)),(LTOK::N_EQ,(0.96,0.91)),(LTOK::GT,(0.97,0.92)),(LTOK::LT,(0.98,0.93)),(LTOK::LT_EQ,(0.99,0.94)),(LTOK::GT_EQ,(0.995,0.945))
        ])});
    pub struct Parser {
        input: Vec<LTOK>,
        idx:usize,
        ast: Option<AST::AstNode>,
    }
    impl Parser{
        fn new(v:Vec<LTOK>) -> Self{
            Self{ast:None,input:v,idx:0}
        }

        fn next(&mut self) -> &LTOK{
            self.idx += 1;
            self.input.get(self.idx - 1).unwrap_or(&LTOK::EOF)
        }
        fn peek(&self) -> &LTOK{
            self.input.get(self.idx).unwrap_or(&LTOK::EOF)
        }
        fn expect(&mut self,e: LTOK) -> bool{
            if &e == self.next(){
                return true;
            }
            false
        }

        fn Parse(&mut self) -> bool {            
            let mut ret:Option<AST::AstNode> = None;
            for i in &self.input{
                match *i{
                    LTOK::EOF => {break;},
                   _ => {return false;}
                }

            }

            self.ast = ret;
            true
        }


    }



}
