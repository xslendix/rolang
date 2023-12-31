use std::{collections::HashMap, fmt::Display, cell::RefCell, rc::Rc, io::{Write, stdin, BufRead}, cmp::Ordering};
use text_io::read;
use anyhow::{Result, anyhow};

use crate::parser::{ASTNode, ASTNodeValue};

#[derive(Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
    CastUnsignedRef(String),
    CastFloatRef(String),
    CastRef(String),
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => a.partial_cmp(b),
            (Object::Float(a), Object::Float(b)) => a.partial_cmp(b),
            (Object::Int(a), Object::Float(b)) => (*a as f64).partial_cmp(b),
            (Object::Float(a), Object::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Object::Bool(a), Object::Bool(b)) => a.partial_cmp(b),
            (Object::String(a), Object::String(b)) => a.partial_cmp(b),
            (Object::Null, Object::Null) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(x) => write!(f, "{}", x),
            Object::Float(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", match x {
                true => "adevărat",
                false => "fals",
            }),
            Object::Null => write!(f, "nul"),
            Object::String(x) => write!(f, "{}", x),
            Object::CastUnsignedRef(x) => write!(f, "RefNat({})", x),
            Object::CastFloatRef(x) => write!(f, "RefFloat({})", x),
            Object::CastRef(x) => write!(f, "Ref({})", x),
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
        if matches!(key, "scrie" | "citește") {
            return None;
        }
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
                    Object::String(x) => Object::String(leftv.to_string() + x.as_str()),
                    Object::Null => left,
                    _ => unimplemented!(),
                },
                Object::Int(leftv) => match right {
                    Object::Int(right) => Object::Int(leftv $operator right),
                    Object::Float(right) => Object::Float(leftv as f64 $operator right),
                    Object::Bool(right) => Object::Int(leftv $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::String(x) => Object::String(leftv.to_string() + x.as_str()),
                    Object::Null => left,
                    _ => unimplemented!(),
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
                    Object::String(x) => Object::String(match leftv {
                        true => String::from("adevărat"),
                        false => String::from("fals"),
                    } + x.as_str()),
                    Object::Null => left,
                    _ => unimplemented!(),
                },
                Object::String(leftv) => match right {
                    Object::Int(right) => Object::String(leftv + &right.to_string()),
                    Object::Float(right) => Object::String(leftv + &right.to_string()),
                    Object::Bool(right) => Object::String(leftv + match right {
                        true => "adevărat",
                        false => "fals",
                    }),
                    Object::String(right) => Object::String(leftv + &right.to_string()),
                    Object::Null => Object::String(leftv + "nul"),
                    _ => unimplemented!(),
                },
                Object::Null => left,
                _ => unimplemented!(),
            };
            Ok(ret)
        }
    };
}

macro_rules! perform_logical_operation {
    ($left:expr, $right:expr, $env:expr, $operator:tt) => {
        {
            let left = eval($left.clone(), Some($env))?;
            let right = eval($right.clone(), Some($env))?;
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
                    Object::String(_) => Object::Null,
                    Object::Null => left,
                    _ => unimplemented!(),
                },
                Object::Int(leftv) => match right {
                    Object::Int(right) => Object::Float(leftv as f64 $operator right as f64),
                    Object::Float(right) => Object::Float(leftv as f64 $operator right),
                    Object::Bool(right) => Object::Int(leftv $operator match right {
                        true => 1,
                        false => 0,
                    }),
                    Object::String(_) => Object::Null,
                    Object::Null => left,
                    _ => unimplemented!(),
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
                    Object::String(_) => Object::Null,
                    Object::Null => left,
                    _ => unimplemented!(),
                },
                Object::String(x) => Object::String(x.repeat(match right {
                    Object::Int(j) => j as usize,
                    Object::Float(j) => j as usize,
                    Object::Bool(j) => match j {
                        true => 1,
                        false => 0,
                    },
                    Object::String(_) => 0,
                    Object::Null => 0,
                    _ => unimplemented!(),
                })),
                Object::Null => left,
                _ => unimplemented!(),
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
        Object::String(_) => true,
        _ => true,
    }
}

fn reverse(s: String) -> String {
    s.chars().rev().collect()
}

pub fn eval(root: Box<ASTNode>, parent: Option<Rc<RefCell<Environment>>>) -> Result<Object> {
    let envb = parent.unwrap();
    match root.value {
        ASTNodeValue::Int(x) => Ok(Object::Int(x)),
        ASTNodeValue::Float(x) => Ok(Object::Float(x)),
        ASTNodeValue::Bool(x) => Ok(Object::Bool(x)),
        ASTNodeValue::String(x) => Ok(Object::String(x)),
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
            if let Ok(v) = val_loopy {
                ret = Ok(v);
            } else if let Err(e) = val_loopy {
                ret = Err(e);
            }
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
        ASTNodeValue::Subtract => {
            if root.children.len() == 1 {
                match eval(root.children[0].clone(), Some(envb.clone()))? {
                    Object::Int(x) => Ok(Object::Int(-x)),
                    Object::Float(x) => Ok(Object::Float(-x)),
                    Object::Bool(x) => Ok(Object::Bool(!x)),
                    Object::String(x) => Ok(Object::String(reverse(x))),
                    Object::Null => Ok(Object::Null),
                    _ => unimplemented!(),
                }
            } else {
                perform_operation!(root.children[0], root.children[1], envb.clone(), -)
            }
        }
        ASTNodeValue::Multiply => perform_operation_term!(root.children[0], root.children[1], envb.clone(), *),
        ASTNodeValue::Divide => {
            if let ASTNodeValue::String(s) = &root.children[0].value {
                Ok(Object::String(s.chars().take(s.chars().count()/match eval(root.children[1].clone(), Some(envb.clone()))? {
                    Object::Int(x) => x as usize,
                    Object::Float(x) => x as usize,
                    _ => return Err(anyhow!("Nu poți împărți un șir de caractere așa.")),
                }).collect()))
            } else if let ASTNodeValue::String(_) = root.children[1].value {
                Err(anyhow!("Nu poți împărți la un șir de caracteretring."))
            } else if root.children[1].value == ASTNodeValue::Null {
                Err(anyhow!("Împărțirea la zero este ilegală."))
            } else {
                perform_operation_term!(root.children[0], root.children[1], envb.clone(), /)
            }
        }
        ASTNodeValue::Mod => {
            if root.children[1].value == ASTNodeValue::Null {
                Err(anyhow!("Modul la zero este ilegal."))
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
                Object::String(_) => Err(anyhow!("Nu poti rotunji în jos un șir de caractere.")),
                Object::Null => Ok(Object::Null),
                _ => unimplemented!(),
            }
        }
        ASTNodeValue::Not => {
            Ok(match eval(root.children[0].clone(), Some(envb.clone()))? {
                Object::Null => Object::Bool(true),
                Object::Bool(x) => Object::Bool(!x),
                Object::Float(_) => Object::Bool(false),
                Object::Int(_) => Object::Bool(false),
                Object::String(_) => Object::Bool(false),
                _ => unimplemented!(),
            })
        }
        ASTNodeValue::And => {
            Ok(Object::Bool(truthy(eval(root.children[0].clone(), Some(envb.clone()))?) && truthy(eval(root.children[0].clone(), Some(envb.clone()))?)))
        }
        ASTNodeValue::Or => {
            Ok(Object::Bool(truthy(eval(root.children[0].clone(), Some(envb.clone()))?) || truthy(eval(root.children[0].clone(), Some(envb.clone()))?)))
        }
        ASTNodeValue::FunctionCall(name) => {
            let lol = &envb.borrow_mut().find_variable(name.as_str(), None);
            match lol {
                None => {
                    match name.as_str() {
                        "scrie" => {
                            let mut written = 0;
                            for node in root.children {
                                let res = eval(node.clone(), Some(envb.clone()))?;
                                written += res.to_string().chars().count();
                                print!("{}", res);
                                std::io::stdout().flush()?;
                            }
                            Ok(Object::Int(written as i64))
                        }
                        "citește" => {
                            for node in root.children {
                                match node.value {
                                    ASTNodeValue::Identifier(x) => {
                                        let mut v = String::new();
                                        stdin().lock().read_line(&mut v)?;
                                        envb.clone().borrow_mut().find_variable(&x, Some(Box::new(Object::String(v))));
                                    }
                                    ASTNodeValue::CastRef(x) => {
                                        let v: i64 = read!("{}");
                                        envb.clone().borrow_mut().find_variable(&x, Some(Box::new(Object::Int(v))));
                                    }
                                    ASTNodeValue::CastFloatRef(x) => {
                                        let v: f64 = read!("{}");
                                        envb.clone().borrow_mut().find_variable(&x, Some(Box::new(Object::Float(v))));
                                    }
                                    ASTNodeValue::CastUnsignedRef(x) => {
                                        let v: u64 = read!("{}");
                                        envb.clone().borrow_mut().find_variable(&x, Some(Box::new(Object::Int(v as i64))));
                                    }
                                    _ => return Err(anyhow!("Citirea formatată nu este implementată."))
                                }
                            }
                            Ok(Object::Null)
                        }
                        _ => Err(anyhow!("Nu există funcția `{}`.", name))
                    }
                }
                Some(x) => {
                    if root.children.len() > 0 {
                        unimplemented!("Funcțiile user-defined sunt neimplementate.")
                    } else {
                        Ok(*x.clone())
                    }
                }
            }
        },
        ASTNodeValue::Illegal => {
            todo!("Neimplementat: {}", root.value)
        },
        ASTNodeValue::CastUnsignedRef(x) => Ok(Object::CastUnsignedRef(x)),
        ASTNodeValue::CastFloatRef(x) => Ok(Object::CastFloatRef(x)),
        ASTNodeValue::CastRef(x) => Ok(Object::CastRef(x)),
    }
}
