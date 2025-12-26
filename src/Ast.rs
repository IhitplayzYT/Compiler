pub mod AST {
    use crate::Lexer_Tok::Lex_Tok::LTOK;
    use std::{cell::{Ref, RefCell}, rc::Rc};
    pub type link<T> = Rc<RefCell<T>>;

    pub enum Expr{
        Int(i64),
        Float(f64),
        String(String),
        Null,
        Ident(String),
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
    }
    pub enum Code{
        Expr,
        Statmnt,
    }
    pub struct AST_Node{
        code : Code,
        children : (Option<link<AST_Node>>,Option<link<AST_Node>>),
    }
    impl AST_Node{
        fn new(c : Code) -> Self{
       Self {code:c,children:(None,None)} 
    }
    }
}
