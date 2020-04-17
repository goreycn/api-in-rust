use std::fmt;
use std::fmt::Formatter;

type Result<T> = std::result::Result<T, CheckError>;


pub trait BeanCheck {
    fn validate(&self) -> Result<()>;
    fn sign_check(&self, uri:&String, token:&String)->bool;
}

#[derive(Debug, Clone)]
pub enum CheckError {
    Simple(String),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CheckError::Simple(txt) => write!(f, "{}", txt),
        }
    }
}
