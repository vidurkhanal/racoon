pub mod builltin_funcs;
mod environment;
mod types;

use std::{cell::RefCell, rc::Rc};

use crate::abstract_tree::{
    BlockOfStatements, Expression, Identifier, Infix, Literal, Prefix, Program, Statement,
};
pub use environment::Environment;
pub use types::Object;

const TRUE_OBJECT: Object = Object::BOOL(true);
const FALSE_OBJECT: Object = Object::BOOL(false);
const NULL_OBJECT: Object = Object::NIL;

#[derive(Debug)]
pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }

    pub fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::BOOL(val) => !!val,
            Object::NIL => false,
            _ => false,
        }
    }

    pub fn is_error(obj: &Object) -> bool {
        match obj {
            Object::ERROR(_) => true,
            _ => false,
        }
    }

    pub fn evaluate(&mut self, program: Program) -> Option<Object> {
        let mut result = None;
        for statement in program.iter() {
            match self.evaluate_statement(statement) {
                Some(Object::RETURN(val)) => return Some(*val),
                Some(Object::ERROR(msg)) => return Some(Object::ERROR(msg)),
                obj => result = obj,
            }
        }
        result
    }

    pub fn evaluate_statement(&mut self, statement: &Statement) -> Option<Object> {
        match statement {
            Statement::Let { name, value } => {
                let value = match self.evaluate_expression(value) {
                    Some(obj) => obj,
                    None => return None,
                };

                if Evaluator::is_error(&value) {
                    Some(value)
                } else {
                    let Identifier { literal, token: _ } = name;
                    self.env.borrow_mut().set(literal.clone(), &value);
                    // Some(value)
                    None
                }
            }

            Statement::Return { return_value } => {
                let val = match self.evaluate_expression(return_value) {
                    Some(val) => val,
                    None => Object::ERROR(format!(
                        "EvaluationError: Could not evaluate the return expression \n --> {:?}",
                        return_value
                    )),
                };

                if Evaluator::is_error(&val) {
                    return Some(val);
                }

                Some(Object::RETURN(Box::new(val)))
            }
            Statement::Expression { expression } => self.evaluate_expression(expression),
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Option<Object> {
        match expression {
            Expression::Identifier(ident) => match self.env.borrow_mut().get(ident.literal.clone()) {
                Some(obj) => Some(obj),
                None => {
                    Some(Object::ERROR(format!(
                    "EvaluationError: The identifier {} has not been declared yet and hence is illegal.",
                    ident.literal
                    )))
            },
            },
            Expression::Literal(literal) => self.evaluate_literal(literal.clone()),
            Expression::Prefix(prefix, right) => {
                let right = self.evaluate_expression(right);
                if right.is_none() {
                    return Some(NULL_OBJECT);
                }
                let right = right.unwrap();
                if Evaluator::is_error(&right) {
                    return Some(right);
                }
                self.evaluate_prefix(prefix.clone(), right)
            }
            Expression::Infix(operator, left, right) => {
                let left = self.evaluate_expression(left);
                if left.is_none() {
                    return Some(NULL_OBJECT);
                }
                let left = left.unwrap();
                if Evaluator::is_error(&left) {
                    return Some(left);
                }
                let right = self.evaluate_expression(right);
                if right.is_none() {
                    return Some(NULL_OBJECT);
                }
                let right = right.unwrap();
                if Evaluator::is_error(&right) {
                    return Some(right);
                }
                self.evaluate_infix(operator.clone(), left, right)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition_object = self.evaluate_expression(&condition);
                if condition_object.is_none() {
                    return Some(Object::ERROR(format!(
                        "EvaluationError: Could not evaluate the given condition\n --> {:?}",
                        condition
                    )));
                }
                let condition_object = condition_object.unwrap();
                if Evaluator::is_error(&condition_object) {
                    return Some(condition_object);
                }
                if Evaluator::is_truthy(&condition_object) {
                    self.evaluate_block_statement(consequence)
                } else if let Some(alt) = alternative {
                    self.evaluate_block_statement(alt)
                } else {
                    None
                }
            }
            // Expression::Function { params, body} => Some(Object::FUNCTION(params.clone(), body.clone(), Rc::new(RefCell::new(Environment::new()))))
            Expression::Function { params, body} => Some(Object::FUNCTION(params.clone(), body.clone(), Rc::clone(&self.env)))
            ,
            Expression::Call { func, args } => Some(self.evaluate_call_expr(func.clone(),(args.clone().unwrap()).to_vec())),
            Expression::Index(left_expr, index_expr) => {
                let left =  self.evaluate_expression(left_expr);
                let index_expr = self.evaluate_expression(index_expr);
                if left.is_some() && index_expr.is_some(){
                self.eval_index_expr(left.unwrap(),index_expr.unwrap())
                }else{
                    None
                }
            },

        }
    }

    fn eval_index_expr(&mut self, left: Object, index_expr: Object) -> Option<Object> {
        match left {
            Object::ARRAY(ref array) => {
                if let Object::INTEGER(i) = index_expr {
                    let length = array.len() as i64;
                    if i < 0 || i >= length {
                        return Some(Object::ERROR(format!(
                            "EvaluationError: Array index out of bounds",
                        )));
                    }

                    match array.get(i as usize) {
                        Some(o) => Some(o.clone()),
                        None => Some(NULL_OBJECT),
                    }
                } else {
                    return Some(Object::ERROR(format!(
                        "EvaluationError: {:?} cannot be used as an array index",
                        index_expr
                    )));
                }
            }
            o => Some(Object::ERROR(format!(
                "EvaluationError: Cannot index a  {}",
                Object::type_of(o)
            ))),
        }
    }
    // fn evaluate_expressions(&mut self, expressions: &Vec<Expression>) -> Option<Vec<Object>> {
    //     let mut result = Vec::<Object>::new();

    //     for expr in expressions {
    //         let eval = self.evaluate_expression(expr);
    //         if eval.is_none() {
    //             return Some(vec![Object::ERROR(format!(
    //                 "EvaluationError: Couldn't evaluate the expression --> {:?}",
    //                 expr
    //             ))]);
    //         }
    //         result.push(match eval.unwrap() {
    //             Object::ERROR(error_msg) => return Some(vec![Object::ERROR(error_msg)]),
    //             obj => obj,
    //         });
    //     }

    //     Some(result)
    // }

    fn evaluate_literal(&mut self, literal: Literal) -> Option<Object> {
        match literal {
            Literal::Int { token: _, value } => {
                return Some(Object::INTEGER(value));
            }
            Literal::String(str) => return Some(Object::STRING(str)),
            Literal::Bool(bool) => match bool {
                true => Some(TRUE_OBJECT),
                false => Some(FALSE_OBJECT),
            },
            Literal::Array(objects) => self.evaluate_array_literal(objects),
            Literal::Hash(_) => Some(NULL_OBJECT),
        }
    }

    fn evaluate_array_literal(&mut self, objects: Vec<Expression>) -> Option<Object> {
        Some(Object::ARRAY(
            objects
                .iter()
                .map(|expr| self.evaluate_expression(expr).unwrap_or(NULL_OBJECT))
                .collect::<Vec<_>>(),
        ))
    }

    fn evaluate_prefix(&mut self, prefix: Prefix, expression: Object) -> Option<Object> {
        match prefix {
            Prefix::Plus => Some(NULL_OBJECT),
            Prefix::Minus => self.evaluate_minus_operator_expression(expression),
            Prefix::Not => self.evaluate_not_operator_expression(expression),
        }
    }
    fn evaluate_infix(&mut self, operator: Infix, left: Object, right: Object) -> Option<Object> {
        match left {
            Object::INTEGER(left_val) => {
                if let Object::INTEGER(right_val) = right {
                    match operator {
                        Infix::Plus => Some(Object::INTEGER(left_val + right_val)),
                        Infix::Minus => Some(Object::INTEGER(left_val - right_val)),
                        Infix::Divide => Some(Object::INTEGER(left_val / right_val)),
                        Infix::Multiply => Some(Object::INTEGER(left_val * right_val)),
                        Infix::Equal => {
                            if left_val == right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::NotEqual => {
                            if left_val != right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::GreaterThanEqual => {
                            if left_val >= right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::GreaterThan => {
                            if left_val > right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::LessThanEqual => {
                            if left_val <= right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::LessThan => {
                            if left_val < right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                    }
                } else {
                    return Some(Object::ERROR(format!(
                        "EvaluationError: {:?} Operator not supported between the two objects {:?} and {:?}",
                        operator,
                        Object::type_of(left),
                        Object::type_of(right)
                    )));
                }
            }
            Object::BOOL(left_val) => {
                if let Object::BOOL(right_val) = right {
                    match operator {
                        Infix::Plus
                        | Infix::Minus
                        | Infix::Divide
                        | Infix::Multiply
                        | Infix::GreaterThanEqual
                        | Infix::GreaterThan
                        | Infix::LessThan
                        | Infix::LessThanEqual => {
                            return Some(Object::ERROR(format!(
                                "EvaluationError: {:?} Operation not supported between boolean expresions.",
                                operator,
                            )))
                        }
                        Infix::Equal => {
                            if left_val == right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::NotEqual => {
                            if left_val != right_val {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                    }
                } else {
                    return Some(Object::ERROR(format!(
                        "EvaluationError: {:?} Operation not supported between the two objects {:?} and {:?}",
                        operator,
                        Object::type_of(left),
                        Object::type_of(right)
                    )));
                }
            }
            Object::NIL | Object::ERROR(_) | Object::RETURN(_) | Object::FUNCTION(_, _, _) => {
                return Some(Object::ERROR(format!(
                    "EvaluationError: {:?} Operator not supported between the two objects {:?} and {:?}",
                    operator,
                    Object::type_of(left),
                    Object::type_of(right)
                )));
            }
            Object::STRING(ref left_str) => {
                if let Object::STRING(ref right_str) = right {
                    match operator {
                        Infix::Plus => {
                            let concat = left_str.clone() + &right_str;
                            Some(Object::STRING(concat))
                        }
                        Infix::Minus
                        | Infix::Divide
                        | Infix::Multiply
                        | Infix::GreaterThanEqual
                        | Infix::GreaterThan
                        | Infix::LessThan
                        | Infix::LessThanEqual => {
                            return Some(Object::ERROR(format!(
                            "EvaluationError: {:?} Operation not supported between string objects.",
                            operator,
                        )))
                        }
                        Infix::Equal => {
                            if left_str == right_str {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                        Infix::NotEqual => {
                            if left_str != right_str {
                                Some(TRUE_OBJECT)
                            } else {
                                Some(FALSE_OBJECT)
                            }
                        }
                    }
                } else {
                    return Some(Object::ERROR(format!(
                    "EvaluationError: {:?} Operation not supported between the two objects {:?} and {:?}",
                    operator,
                    Object::type_of(left),
                    Object::type_of(right)
                )));
                }
            }
            Object::BUILTIN { arity:_, builtInFunc:_ }=> Some(Object::ERROR(format!(
                "EvaluationError: {:?} Operation not supported between the two objects {:?} and {:?}",
                operator,
                Object::type_of(left),
                Object::type_of(right)
            ))),
            Object::ARRAY(_) => Some(Object::ERROR(format!(
                "EvaluationError: {:?} Operation not supported between the two objects {:?} and {:?}",
                operator,
                Object::type_of(left),
                Object::type_of(right)
            ))),
        }
    }

    fn evaluate_not_operator_expression(&mut self, expression: Object) -> Option<Object> {
        match expression {
            Object::BOOL(true) => Some(FALSE_OBJECT),
            Object::BOOL(false) => Some(TRUE_OBJECT),
            Object::NIL => Some(TRUE_OBJECT),
            _ => Some(FALSE_OBJECT),
        }
    }

    fn evaluate_minus_operator_expression(&mut self, expression: Object) -> Option<Object> {
        match expression {
            Object::INTEGER(val) => Some(Object::INTEGER(-val)),
            _ => Some(Object::ERROR(format!(
                "EvaluationError: MINUS operator cannot be used for the type {}",
                Object::type_of(expression)
            ))),
        }
    }

    fn evaluate_block_statement(&mut self, statements: &BlockOfStatements) -> Option<Object> {
        let mut result = None;
        for statement in statements {
            match self.evaluate_statement(statement) {
                Some(Object::RETURN(val)) => return Some(*val),
                Some(Object::ERROR(_)) => return result,
                obj => result = obj,
            }
        }
        result
    }

    fn evaluate_call_expr(&mut self, func: Box<Expression>, args: Vec<Expression>) -> Object {
        let args_supplied = args
            .iter()
            .map(|e| self.evaluate_expression(e).unwrap_or(NULL_OBJECT))
            .collect::<Vec<_>>();

        let (args_expected, body, env) = match self.evaluate_expression(&func) {
            Some(Object::FUNCTION(params, body, env)) => (params, body, env),
            Some(Object::BUILTIN { arity, builtInFunc }) => {
                if arity < 0 || arity == args_supplied.len() as i16 {
                    return builtInFunc(args_supplied);
                } else {
                    return Object::ERROR(format!(
                        "EvaluationError: Expected {} arguments, but {} were supplied.",
                        arity,
                        args_supplied.len()
                    ));
                }
            }

            Some(obj) => {
                return Object::ERROR(format!(
                    "EvaluationError: Expected function instead received {}",
                    Object::type_of(obj)
                ))
            }
            None => return NULL_OBJECT,
        };

        if args_supplied.len() != args_expected.len() {
            return Object::ERROR(format!(
                "EvaluationError: Expected {} arguments, but {} were supplied.",
                args_expected.len(),
                args_supplied.len()
            ));
        }

        let current_env = Rc::clone(&self.env);
        let mut closure_env = Environment::new_with_outer(Rc::clone(&env));
        for (i, _) in args_supplied.iter().enumerate() {
            let Identifier { literal, token: _ } = args_expected[i].clone();
            closure_env.set(literal, &args_supplied[i]);
        }

        self.env = Rc::new(RefCell::new(closure_env));
        let obj = self.evaluate_block_statement(&body);
        self.env = current_env;

        match obj {
            Some(obj) => obj,
            None => NULL_OBJECT,
        }
    }
}
