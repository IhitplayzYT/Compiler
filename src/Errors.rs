pub mod Err{

#[derive(Debug,Clone)]
pub enum ParserError{
UnexpectedToken{expected:String,got:String},
Invalid_Code,
Custom(String),
}


pub type Parser_ret<T> = Result<T,ParserError>;
}