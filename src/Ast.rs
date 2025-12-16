pub mod AST {
    use crate::Lexer_Tok::Lex_Tok::LTOK;
    use std::{cell::RefCell, rc::Rc};
    pub type link = Rc<RefCell<AstNode>>;

    pub enum AstNode {
        Int(i64),
        Float(f64),
        String(String),
        Null,
        Ident(String),
        Binary_op {
            op: LTOK, // Represents the Node for an operator
            left: link,
            right: link,
        },
        Let {
            name: String, // Deals with let,mut and const
            mutable: bool,
            value: link,
        },
        If {
            cond: link,
            then_branch: Vec<link>, // Deals with if and else
            else_branch: Option<Vec<link>>,
        },
        While {
            cond: link, // While loop
            body: Vec<link>,
        },
        For {
            init: Vec<link>,
            cond: Option<link>, // For loop SYNTAX : C and Rust hybrid type
            step: Option<link>,
            body: Vec<link>,
        },
    }
}
