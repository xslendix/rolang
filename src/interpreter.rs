use std::{collections::HashMap, fmt::Display, cell::RefCell, rc::Rc};
use anyhow::{Result, anyhow};

use crate::parser::{ASTNode, ASTNodeValue};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Object {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(x) => write!(f, "Int({})", x),
            Object::Float(x) => write!(f, "Float({})", x),
            Object::Bool(x) => write!(f, "Bool({})", x),
            Object::Null => write!(f, "Nul"),
        }
    }
}

pub struct Environment {
    pub syms: HashMap<String, Box<Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            syms: HashMap::new(),
        }
    }

    pub fn find_variable(&mut self, key: &str, value: Option<Box<Object>>) -> Option<Box<Object>> {
        if let Some(obj) = self.syms.get(key) {
            if let Some(val) = value {
                self.syms.remove(key);
                self.syms.insert(key.to_string(), val.clone());
                Some(val)
            } else {
                Some(obj.clone())
            }
        } else {
            match value {
                Some(v) => {
                    self.syms.remove(key);
                    self.syms.insert(key.to_string(), v.clone());
                    Some(v)
                }
                None => None
            }
        }
    }
}

macro_rules! perform_operation {
    ($left:expr, $right:expr, $env:expr, $operator:tt) => {
        {
            let left = eval($left.clone(), Some($env))?;
            let right = eval($right.clone(), Some($env))?;
            let ret: Object = match left {
                Object::Float(leftv) => match right {
                    Object::Int(right) => Object::Float(leftv $operator right as f64),
                    Object::Float(right) => Object::Float(leftv $operator right),
                    Object::Bool(right) => Object::Float(leftv $operator match right {
                        true => 1.0,
                        false => 0.0,
                    }),
                    Object::Null => left,
                },
                Object::Int(leftv) => match right {
                    Object::Int(right) => Object::Int(leftv $operator right),
                    Object::Float(right) => Object::Float(leftv as f64 $operator right),
                    Object::Bool(right) => Object::Int(leftv $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::Null => left,
                },
                Object::Bool(leftv) => match right {
                    Object::Int(right) => Object::Int(match leftv {
                        true => 1,
                        false => 0,
                    } $operator right),
                    Object::Float(right) => Object::Float(match leftv {
                        true => 1.0,
                        false => 0.0,
                    } $operator right),
                    Object::Bool(right) => Object::Int(match leftv {
                        true => 1,
                        false => 0,
                    } $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::Null => left,
                },
                Object::Null => left,
            };
            Ok(ret)
        }
    };
}

macro_rules! perform_logical_operation {
    ($left:expr, $right:expr, $env:expr, $operator:tt) => {
        {
            let left = (eval($left.clone(), Some($env))?);
            let right = (eval($right.clone(), Some($env))?);
            Ok(Object::Bool(left $operator right))
        }
    };
}

macro_rules! perform_operation_term {
    ($left:expr, $right:expr, $env:expr, $operator:tt) => {
        {
            let left = eval($left.clone(), Some($env))?;
            let right = eval($right.clone(), Some($env))?;
            let ret: Object = match left {
                Object::Float(leftv) => match right {
                    Object::Int(right) => Object::Float(leftv $operator right as f64),
                    Object::Float(right) => Object::Float(leftv $operator right),
                    Object::Bool(right) => Object::Float(leftv $operator match right {
                        true => 1.0,
                        false => 0.0,
                    }),
                    Object::Null => left,
                },
                Object::Int(leftv) => match right {
                    Object::Int(right) => Object::Float(leftv as f64 $operator right as f64),
                    Object::Float(right) => Object::Float(leftv as f64 $operator right),
                    Object::Bool(right) => Object::Int(leftv $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::Null => left,
                },
                Object::Bool(leftv) => match right {
                    Object::Int(right) => Object::Int(match leftv {
                        true => 1,
                        false => 0,
                    } $operator right),
                    Object::Float(right) => Object::Float(match leftv {
                        true => 1.0,
                        false => 0.0,
                    } $operator right),
                    Object::Bool(right) => Object::Int(match leftv {
                        true => 1,
                        false => 0,
                    } $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::Null => left,
                },
                Object::Null => left,
            };
            Ok(ret)
        }
    };
}

pub fn truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Int(x) => x != 0,
        Object::Bool(x) => x,
        Object::Float(x) => x != 0.0,
    }
}

pub fn eval<'a>(root: Box<ASTNode>, parent: Option<Rc<RefCell<Environment>>>) -> Result<Object> {
    let envb = parent.clone().unwrap();
    match root.value {
        ASTNodeValue::Int(x) => Ok(Object::Int(x)),
        ASTNodeValue::Float(x) => Ok(Object::Float(x)),
        ASTNodeValue::Bool(x) => Ok(Object::Bool(x)),
        ASTNodeValue::Null => Ok(Object::Null),
        ASTNodeValue::Identifier(x) => {
            match &envb.borrow_mut().find_variable(x.as_str(), None) {
                Some(obj) => Ok(*obj.clone()),
                None => Ok(Object::Null),
            }
        }
        ASTNodeValue::If => {
            let cond = eval(root.children[0].clone(), Some(envb.clone()))?;
            if truthy(cond) {
                eval(root.children[1].clone(), Some(envb.clone()))
            } else {
                if root.children.len() == 3 {
                    eval(root.children[2].clone(), Some(envb.clone()))
                } else {
                    Ok(Object::Null)
                }
            }
        }
        ASTNodeValue::While => {
            let mut ret = Ok(Object::Null);
            while truthy(eval(root.children[0].clone(), Some(envb.clone()))?) {
                ret = Ok(eval(root.children[1].clone(), Some(envb.clone()))?)
            }
            ret
        }
        ASTNodeValue::Until => {
            let mut ret = Ok(Object::Null);
            while !truthy(eval(root.children[0].clone(), Some(envb.clone()))?) {
                ret = Ok(eval(root.children[1].clone(), Some(envb.clone()))?)
            }
            ret
        }
        ASTNodeValue::Execute => {
            let mut ret = Ok(eval(root.children[0].clone(), Some(envb.clone()))?);
            let mut loopy = root.children[1].clone();
            loopy.children.push(root.children[0].clone());
            let val_loopy = eval(loopy, Some(envb.clone()));
            ret = val_loopy;
            ret
        }
        ASTNodeValue::For => {
            let varname = &root.children[0].children[0];
            let varname = match &varname.value {
                ASTNodeValue::Identifier(x) => x,
                _ => unreachable!(),
            };
            let start = match eval(root.children[0].clone(), Some(envb.clone()))? {
                Object::Int(x) => x,
                _ => unreachable!("Dacă acceptă doar Int"),
            };
            let end = match eval(root.children[1].clone(), Some(envb.clone()))? {
                Object::Int(x) => x,
                _ => unreachable!("Dacă acceptă doar Int"),
            };
            let step = if root.children.len() == 4 {
                match eval(root.children[2].clone(), Some(envb.clone()))? {
                    Object::Int(x) => x,
                    _ => unreachable!("Dacă acceptă doar Int"),
                }
            } else {
                1
            };
            let mut ret = Ok(Object::Null);

            for n in (start..=end).step_by(step.try_into().unwrap()) {
                envb.borrow_mut().find_variable(varname, Some(Box::new(Object::Int(n))));
                if root.children.len() == 4 {
                    ret = eval(root.children[3].clone(), Some(envb.clone()));
                } else {
                    ret = eval(root.children[2].clone(), Some(envb.clone()));
                }
                envb.borrow_mut().find_variable(varname, Some(Box::new(Object::Int(n))));
            }
            ret
        }
        ASTNodeValue::Set => {
            if let ASTNodeValue::Identifier(ident) = &root.children[0].value {
                let value = eval(root.children[1].clone(), Some(envb.clone()))?;
                let res = envb.borrow_mut().find_variable(ident, Some(Box::new(value)));
                if let Some(val) = res {
                    Ok(*val.clone())
                } else {
                    Ok(Object::Null)
                }
            } else {
                unreachable!()
            }
        }
        ASTNodeValue::Program => {
            let mut ret: Object = Object::Null;
            for child in root.children {
                ret = eval(child, Some(envb.clone()))?;
            }
            Ok(ret)
        }
        ASTNodeValue::Equal => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), ==),
        ASTNodeValue::NotEqual => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), !=),
        ASTNodeValue::LessThan => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), <),
        ASTNodeValue::LessThanEqual => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), <=),
        ASTNodeValue::GreaterThan => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), >),
        ASTNodeValue::GreaterThanEqual => perform_logical_operation!(root.children[0], root.children[1], envb.clone(), >=),
        ASTNodeValue::Add => perform_operation!(root.children[0], root.children[1], envb.clone(), +),
        ASTNodeValue::Subtract => perform_operation!(root.children[0], root.children[1], envb.clone(), -),
        ASTNodeValue::Multiply => perform_operation_term!(root.children[0], root.children[1], envb.clone(), *),
        ASTNodeValue::Divide => {
            if root.children[1].value == ASTNodeValue::Null {
                Err(anyhow!("Division by zero is illegal."))
            } else {
                perform_operation_term!(root.children[0], root.children[1], envb.clone(), /)
            }
        }
        ASTNodeValue::Mod => {
            if root.children[1].value == ASTNodeValue::Null {
                Err(anyhow!("Modulo by zero is illegal."))
            } else {
                perform_operation!(root.children[0], root.children[1], envb.clone(), %)
            }
        }
        ASTNodeValue::Floor => {
            let val = eval(root.children[0].clone(), Some(envb.clone()))?;
            match val {
                Object::Int(x) => Ok(Object::Int(x)),
                Object::Float(x) => Ok(Object::Int(x.floor() as i64)),
                Object::Bool(_) => Err(anyhow!("Nu poti rotunji în jos un bool.")),
                Object::Null => Ok(Object::Null),
            }
        }
        ASTNodeValue::Not => {
            Ok(match eval(root.children[0].clone(), Some(envb.clone()))? {
                Object::Null => Object::Bool(true),
                Object::Bool(x) => Object::Bool(!x),
                Object::Float(_) => Object::Bool(false),
                Object::Int(_) => Object::Bool(false),
            })
        }
        ASTNodeValue::And => {
            Ok(Object::Bool(truthy(eval(root.children[0].clone(), Some(envb.clone()))?) && truthy(eval(root.children[0].clone(), Some(envb.clone()))?)))
        }
        ASTNodeValue::Or => {
            Ok(Object::Bool(truthy(eval(root.children[0].clone(), Some(envb.clone()))?) || truthy(eval(root.children[0].clone(), Some(envb.clone()))?)))
        }
        ASTNodeValue::FunctionCall(_) | ASTNodeValue::Illegal => {
            todo!("Unimplemented: {}", root.value)
        }
    }
}