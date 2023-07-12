use std::{mem, fmt::Display};

use crate::lexer::{Token, Lexer};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum ASTNodeValue {
    Identifier(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    Program,

    Set,

    If,
    For,
    Execute,
    While,
    Until,

    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,

    Not,
    And,
    Or,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    Floor,
    FunctionCall(String),

    Illegal,
}

#[derive(Clone)]
pub struct ASTNode {
    pub value: ASTNodeValue,
    pub children: Vec<Box<ASTNode>>,
}

impl ASTNode {
    pub fn from_token(token: Token) -> ASTNode {
        ASTNode {
            value: match token {
                Token::Identifier(x) => ASTNodeValue::Identifier(x),
                Token::Int(x) => ASTNodeValue::Int(x),
                Token::Float(x) => ASTNodeValue::Float(x),
                Token::True => ASTNodeValue::Bool(true),
                Token::False => ASTNodeValue::Bool(false),
                Token::Null => ASTNodeValue::Null,

                Token::If => ASTNodeValue::If,
                Token::For => ASTNodeValue::For,
                Token::Execute => ASTNodeValue::Execute,
                Token::While => ASTNodeValue::While,
                Token::Until => ASTNodeValue::Until,

                Token::Add => ASTNodeValue::Add,
                Token::Subtract => ASTNodeValue::Subtract,
                Token::Multiply => ASTNodeValue::Multiply,
                Token::Divide => ASTNodeValue::Divide,
                Token::Mod => ASTNodeValue::Mod,

                Token::Not => ASTNodeValue::Not,
                Token::And => ASTNodeValue::And,
                Token::Or => ASTNodeValue::Or,

                Token::Equal => ASTNodeValue::Equal,
                Token::NotEqual => ASTNodeValue::NotEqual,
                Token::LessThan => ASTNodeValue::LessThan,
                Token::GreaterThan => ASTNodeValue::GreaterThan,
                Token::LessThanEqual => ASTNodeValue::LessThanEqual,
                Token::GreaterThanEqual => ASTNodeValue::GreaterThanEqual,

                Token::Illegal => ASTNodeValue::Illegal,

                _ => unreachable!("Unimplemented token: {}", token)
            },
            children: Vec::new()
        }
    }

    pub fn from(value: ASTNodeValue) -> ASTNode {
        ASTNode {
            value,
            children: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, mut ident: String, last: bool) {
        print!("{}", ident);
        if last {
            print!("\\-");
            ident.push_str("  ");
        } else {
            print!("|-");
            ident.push_str("| ");
        }
        println!(" {}", match &self.value {
            ASTNodeValue::Identifier(x) => format!("Identifier \x1b[40G{}", x),
            ASTNodeValue::Int(x) => format!("Int \x1b[40G{}", x),
            ASTNodeValue::Float(x) => format!("Float \x1b[40G{}", x),
            ASTNodeValue::Bool(x) => format!("Bool \x1b[40G{}", x),
            ASTNodeValue::Null => "Null".to_string(),
            ASTNodeValue::Program => "Program".to_string(),

            ASTNodeValue::Set => "Set".to_string(),

            ASTNodeValue::If => "If".to_string(),
            ASTNodeValue::For => "For".to_string(),
            ASTNodeValue::Execute => "Execute".to_string(),
            ASTNodeValue::While => "While".to_string(),
            ASTNodeValue::Until => "Until".to_string(),

            ASTNodeValue::Add => "Add".to_string(),
            ASTNodeValue::Subtract => "Subtract".to_string(),
            ASTNodeValue::Multiply => "Multiply".to_string(),
            ASTNodeValue::Divide => "Divide".to_string(),
            ASTNodeValue::Mod => "Mod".to_string(),

            ASTNodeValue::Not => "Not".to_string(),
            ASTNodeValue::And => "And".to_string(),
            ASTNodeValue::Or => "Or".to_string(),

            ASTNodeValue::Equal => "Equal".to_string(),
            ASTNodeValue::NotEqual => "NotEqual".to_string(),
            ASTNodeValue::LessThan => "LessThan".to_string(),
            ASTNodeValue::GreaterThan => "GreaterThan".to_string(),
            ASTNodeValue::LessThanEqual => "LessThanEqual".to_string(),
            ASTNodeValue::GreaterThanEqual => "GreaterThanEqual".to_string(),

            ASTNodeValue::Floor => "Floor".to_string(),
            ASTNodeValue::FunctionCall(x) => format!("FunctionCall\x1b[40G{}", x),

            ASTNodeValue::Illegal => "Illegal".to_string(),
        });
        let mut i = 0;
        for node in &self.children {
            node.print(ident.clone(), i == self.children.len() - 1);
            i += 1;
        }
    }
}

impl Display for ASTNodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNodeValue::Identifier(x) => write!(f, "Identifier({})", x),
            ASTNodeValue::Int(x) => write!(f, "Int({})", x),
            ASTNodeValue::Float(x) => write!(f, "Float({})", x),
            ASTNodeValue::Bool(x) => write!(f, "Bool({})", x),
            ASTNodeValue::Null => write!(f, "Null"),
            ASTNodeValue::Program => write!(f, "Program"),

            ASTNodeValue::Set => write!(f, "Set"),

            ASTNodeValue::If => write!(f, "If"),
            ASTNodeValue::For => write!(f, "For"),
            ASTNodeValue::Execute => write!(f, "Execute"),
            ASTNodeValue::While => write!(f, "While"),
            ASTNodeValue::Until => write!(f, "Until"),

            ASTNodeValue::Add => write!(f, "Add"),
            ASTNodeValue::Subtract => write!(f, "Subtract"),
            ASTNodeValue::Multiply => write!(f, "Multiply"),
            ASTNodeValue::Divide => write!(f, "Divide"),
            ASTNodeValue::Mod => write!(f, "Mod"),

            ASTNodeValue::Not => write!(f, "Not"),
            ASTNodeValue::And => write!(f, "And"),
            ASTNodeValue::Or => write!(f, "Or"),

            ASTNodeValue::Equal => write!(f, "Equal"),
            ASTNodeValue::NotEqual => write!(f, "NotEqual"),
            ASTNodeValue::LessThan => write!(f, "LessThan"),
            ASTNodeValue::GreaterThan => write!(f, "GreaterThan"),
            ASTNodeValue::LessThanEqual => write!(f, "LessThanEqual"),
            ASTNodeValue::GreaterThanEqual => write!(f, "GreaterThanEqual"),

            ASTNodeValue::Floor => write!(f, "Floor"),
            ASTNodeValue::FunctionCall(x) => write!(f, "FunctionCall({})", x),

            ASTNodeValue::Illegal => write!(f, "Illegal"),
        }
    }
}

pub struct Parser {
    pub lexer: Lexer,
    c: Token,
    n: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            c: Token::Illegal,
            n: Token::Illegal,
            errors: Vec::new()
        };

        parser.next();
        parser.next();

        parser
    }

    fn next(&mut self) {
        self.c = self.n.clone();
        self.n = self.lexer.next().unwrap_or_else(|err| {
            self.errors.push(err.to_string());
            Token::Illegal
        });
    }

    fn next_prev(&mut self) -> Token {
        let old = self.c.clone();
        self.next();
        old
    }

    fn is(&mut self, ttype: Token) -> bool {
        mem::discriminant(&ttype) == mem::discriminant(&self.c)
    }

    fn accept(&mut self, ttype: Token) -> bool {
        if self.is(ttype) {
            self.next();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, ttype: Token) -> bool {
        if self.accept(ttype.clone()) {
            return true;
        }
        self.errors.push(format!("Expected {}, got {}", ttype, self.c).to_string());
        false
    }

    fn function_call(&mut self) -> Box<ASTNode> {
        let mut node = ASTNode::from(ASTNodeValue::FunctionCall(match &self.next_prev() {
            Token::Identifier(x) => x.to_string(),
            _ => unreachable!()
        }));
        self.expect(Token::LParen);
        while !self.is(Token::RParen) {
            node.children.push(self.expr());
            if !self.is(Token::RParen) {
                self.expect(Token::Comma);
            }
        }
        self.expect(Token::RParen);
        Box::new(node)
    }

    fn factor(&mut self) -> Box<ASTNode> {
        if self.is(Token::Identifier(String::new())) && self.n == Token::LParen {
            self.function_call()
        } else if self.is(Token::Identifier(String::new())) || self.is(Token::Int(0)) || self.is(Token::Float(0.0)) || self.is(Token::Null) || self.is(Token::False) || self.is(Token::True) {
            let ret = Box::new(ASTNode::from_token(self.c.clone()));
            self.next();
            ret
        } else if self.accept(Token::LParen) {
            let ret = self.math_expr();
            self.expect(Token::RParen);
            ret
        } else {
            self.errors.push(format!("Illegal token: {}", self.c));
            self.next();
            Box::new(ASTNode::from_token(Token::Illegal))
        }
    }

    fn term(&mut self) -> Box<ASTNode> {
        let mut node: Box<ASTNode>;
        if self.is(Token::Subtract) || self.is(Token::Not) {
            node = Box::new(ASTNode::from_token(self.c.clone()));
            node.children.push(self.term());
            return node;
        } else {
            node = self.factor();
        }

        while matches!(self.c, Token::Multiply | Token::Divide | Token::Mod) {
            let mut new = ASTNode::from_token(self.next_prev());
            let right = self.factor();
            if right.value == ASTNodeValue::Illegal {
                self.errors.push(String::from("Missing expression"));
                break;
            }
            new.children.push(node);
            new.children.push(right);
            node = Box::new(new);
        }

        node
    }

    fn math_expr(&mut self) -> Box<ASTNode> {
        if self.accept(Token::FloorStart) {
            let mut node = ASTNode::from(ASTNodeValue::Floor);
            node.children.push(self.math_expr());
            self.expect(Token::FloorEnd);
            return Box::new(node);
        }

        let mut node = self.term();
        while matches!(self.c, Token::Add | Token::Subtract) {
            let mut new = ASTNode::from_token(self.next_prev());
            let right = self.math_expr();
            if right.value == ASTNodeValue::Illegal {
                self.errors.push(String::from("Missing expression"));
                break;
            }
            new.children.push(node);
            new.children.push(right);
            node = Box::new(new);
        }
        node
    }

    fn logical_expr(&mut self) -> Box<ASTNode> {
        let mut node = self.math_expr();
        while matches!(self.c, Token::And |
                Token::Or |
                Token::Equal |
                Token::NotEqual |
                Token::LessThan |
                Token::GreaterThan |
                Token::LessThanEqual |
                Token::GreaterThanEqual) {
            let mut new = ASTNode::from_token(self.next_prev());
            let right = self.logical_expr();
            if right.value == ASTNodeValue::Illegal {
                self.errors.push(String::from("Missing expression"));
                break;
            }
            new.children.push(node);
            new.children.push(right);
            node = Box::new(new);
        }
        node
    }

    fn expr(&mut self) -> Box<ASTNode> {
        self.logical_expr()
    }

    fn pif(&mut self) -> Box<ASTNode> {
        self.expect(Token::If);
        let mut node = ASTNode::from_token(Token::If);

        let expr = self.expr();
        if expr.value == ASTNodeValue::Illegal {
            self.errors.push(String::from("Expected condition"));
            return Box::new(ASTNode::from_token(Token::Illegal));
        }

        node.children.push(expr);
        self.expect(Token::Then);

        node.children.push(self.prog(true, true, false));

        if self.accept(Token::Else) {
            node.children.push(self.prog(true, false, false));
        }

        Box::new(node)
    }

    fn pforheader(&mut self) -> Box<ASTNode> {
        self.expect(Token::For);
        let mut node = ASTNode::from_token(Token::For);
        if !self.is(Token::Identifier(String::new())) && self.n != Token::Set {
            self.errors.push(String::from("Expected var def"));
            return Box::new(ASTNode::from_token(Token::Illegal));
        }
        let set = self.set();
        node.children.push(set);
        if !self.expect(Token::Comma) {
            return Box::new(ASTNode::from_token(Token::Illegal));
        }
        node.children.push(self.expr());
        if self.accept(Token::Comma) {
            node.children.push(self.expr());
        }
        Box::new(node)
    }

    fn pwhileheader(&mut self) -> Box<ASTNode> {
        self.expect(Token::While);
        let mut node = ASTNode::from_token(Token::While);
        node.children.push(self.stmt());
        Box::new(node)
    }

    fn puntilheader(&mut self) -> Box<ASTNode> {
        self.expect(Token::Until);
        let mut node = ASTNode::from_token(Token::Until);
        node.children.push(self.stmt());
        Box::new(node)
    }

    fn execute(&mut self) -> Box<ASTNode> {
        self.expect(Token::Execute);
        let mut node = ASTNode::from_token(Token::Execute);
        node.children.push(self.prog(true, false, true));
        if self.is(Token::While) {
            node.children.push(self.pwhileheader());
        } else if self.is(Token::Until) {
            node.children.push(self.puntilheader());
        } else if self.is(Token::For) {
            node.children.push(self.pforheader());
        }
        Box::new(node)
    }

    fn pfor(&mut self) -> Box<ASTNode> {
        let mut node = self.pforheader();
        self.expect(Token::Execute);
        node.children.push(self.prog(true, false, false));
        node
    }

    fn pwhile(&mut self) -> Box<ASTNode> {
        let mut node = self.pwhileheader();
        self.expect(Token::Execute);
        node.children.push(self.prog(true, false, false));
        node
    }

    fn until(&mut self) -> Box<ASTNode> {
        let mut node = self.puntilheader();
        self.expect(Token::Execute);
        node.children.push(self.prog(true, false, false));
        node
    }

    fn set(&mut self) -> Box<ASTNode> {
        let identifier = ASTNode::from_token(self.next_prev());
        self.expect(Token::Set);
        let mut node = ASTNode::from(ASTNodeValue::Set);
        node.children.push(Box::new(identifier));
        node.children.push(self.expr());
        Box::new(node)
    }

    fn stmt(&mut self) -> Box<ASTNode> {
        if self.is(Token::If) {
            self.pif()
        } else if self.is(Token::Execute) {
            self.execute()
        } else if self.is(Token::For) {
            self.pfor()
        } else if self.is(Token::While) {
            self.pwhile()
        } else if self.is(Token::Until) {
            self.until()
        } else if self.is(Token::Identifier(String::new())) && self.n == Token::Set {
            self.set()
        } else {
            self.expr()
        }
    }

    fn prog(&mut self, in_block: bool, in_if: bool, in_execute: bool) -> Box<ASTNode> {
        let mut prog = Box::new(ASTNode::from(ASTNodeValue::Program));
        while self.c != Token::EOF && !(in_if && self.is(Token::Else)) && !(matches!(self.c, Token::While | Token::For | Token::Until) && in_execute) {
            let stmt = self.stmt();

            if stmt.value != ASTNodeValue::Illegal {
                prog.children.push(stmt);
            }
            if in_block && self.is(Token::BlockEnd) {
                self.next();
                break;
            }
        }
        prog
    }

    pub fn parse(&mut self) -> Box<ASTNode> {
        self.prog(false, false, false)
    }
}

