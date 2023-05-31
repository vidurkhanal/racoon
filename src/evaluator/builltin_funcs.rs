use std::collections::HashMap;

use super::Object;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(
        String::from("len"),
        Object::BUILTIN {
            arity: 1,
            builtInFunc: len,
        },
    );
    builtins.insert(
        String::from("head"),
        Object::BUILTIN {
            arity: 1,
            builtInFunc: head,
        },
    );
    builtins.insert(
        String::from("last"),
        Object::BUILTIN {
            arity: 1,
            builtInFunc: last,
        },
    );
    builtins.insert(
        String::from("tail"),
        Object::BUILTIN {
            arity: 1,
            builtInFunc: tail,
        },
    );
    builtins.insert(
        String::from("push"),
        Object::BUILTIN {
            arity: 2,
            builtInFunc: push,
        },
    );
    builtins
}

pub fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(format!("Expected 1 argument but received {}", args.len()));
    }
    match &args[0] {
        Object::STRING(str_val) => Object::INTEGER(str_val.len() as i64),
        Object::ARRAY(arr) => Object::INTEGER(arr.len() as i64),
        _ => Object::ERROR(format!(
            "SyntaxError: len() function cannot be used for {}",
            Object::type_of(args[0].clone())
        )),
    }
}

pub fn head(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(format!("Expected 1 argument but received {}", args.len()));
    }
    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.len()>0{
                return arr.get(0).unwrap().clone();
            }
            Object::ERROR(format!("LogicalError: The array has no elements or hasn't been initialized yet."))
        },
        _ => Object::ERROR(format!(
            "SyntaxError: first() function cannot be used for {}. It can only be used for Array type.",
            Object::type_of(args[0].clone())
        )),
    }
}

pub fn last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(format!("Expected 1 argument but received {}", args.len()));
    }
    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.len()>0{
                return arr.get(arr.len()-1).unwrap().clone();
            }
            Object::ERROR(format!("LogicalError: The array has no elements or hasn't been initialized yet."))
        },
        _ => Object::ERROR(format!(
            "SyntaxError: last() function cannot be used for {}. It can only be used for Array type.",
            Object::type_of(args[0].clone())
        )),
    }
}

pub fn tail(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(format!("Expected 1 argument but received {}", args.len()));
    }
    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.len()>0{
                return Object::ARRAY(arr[1..].to_vec());
            }
            Object::ERROR(format!("LogicalError: The array has no elements or hasn't been initialized yet."))
        },
        _ => Object::ERROR(format!(
            "SyntaxError: last() function cannot be used for {}. It can only be used for Array type.",
            Object::type_of(args[0].clone())
        )),
    }
}

fn push(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::ARRAY(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Object::ARRAY(new_arr)
        }
        o => Object::ERROR(format!(
            "argument to `push` must be array. got {}",
            Object::type_of(o.clone())
        )),
    }
}
