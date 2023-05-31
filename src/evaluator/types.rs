use std::{cell::RefCell, fmt, rc::Rc};

use crate::abstract_tree::{BlockOfStatements, Identifier};

use super::Environment;

pub type BuiltInFunc = fn(Vec<Object>) -> Object;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    INTEGER(i64),
    BOOL(bool),
    NIL,
    ERROR(String),
    RETURN(Box<Object>),
    FUNCTION(Vec<Identifier>, BlockOfStatements, Rc<RefCell<Environment>>),
    STRING(String),
    BUILTIN {
        arity: i16,
        builtInFunc: BuiltInFunc,
    },
    ARRAY(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::INTEGER(n) => write!(f, "{}", n),
            Object::BOOL(b) => write!(f, "{}", b),
            Object::NIL => write!(f, "\n"),
            Object::ERROR(s) => write!(f, "Error: {}", s),
            Object::RETURN(obj) => write!(f, "Return({})", obj),
            Object::FUNCTION(params, body, _) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", param)?;
                }
                write!(f, ") {:?}", body)?;
                Ok(())
            }
            Object::STRING(s) => write!(f, "\"{}\"", s),
            Object::BUILTIN { arity, builtInFunc } => {
                write!(f, "builtin ({}) {:?}", arity, builtInFunc)
            }
            Object::ARRAY(arr) => {
                write!(f, "[")?;
                for (i, obj) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Object {
    pub fn type_of(object: Object) -> String {
        match object {
            Object::INTEGER(_) => String::from("int64"),
            Object::BOOL(_) => String::from("bool"),
            Object::NIL => String::from("nil"),
            Object::ERROR(_) => String::from("Error"),
            Object::STRING(_) => String::from("String"),
            Object::RETURN(ref value) => format!("{:#?}", value),
            Object::FUNCTION(params, _, _) => format!("func ({:#?})", params),
            Object::BUILTIN {
                arity: _,
                builtInFunc: _,
            } => format!("builtin_func"),
            Object::ARRAY(_) => format!("Array"),
        }
    }
}
