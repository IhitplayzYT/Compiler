pub mod Compiler{
use crate::Frontend::Frontend::Frontend;
pub struct Compiler{
    frontend:Frontend,
}

impl Compiler{
pub fn new(file_path : &str) -> Self{
    Self{frontend:Frontend::new(file_path.to_string())}
}

}
}