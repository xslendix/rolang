use std::{char, fmt::Display};

use anyhow::{Result, anyhow};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    True,
    False,
    Null,

    CastUnsignedRef,
    CastFloatRef,
    CastRef,

    Comma,
    SColon,

    If,
    Then,
    Else,
    For,
    Execute,
    Repeat,
    While, // câttimp
    Until, // pânăcând

    BlockEnd,
    FloorStart,
    FloorEnd,

    LParen,
    RParen,

    Set,

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

    Illegal,
    EOF,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        match self {
            Token::Identifier(x) => Token::Identifier(x.clone()),
            Token::Int(x) => Token::Int(x.clone()),
            Token::Float(x) => Token::Float(x.clone()),
            Token::String(x) => Token::String(x.clone()),
            Token::True => Token::True,
            Token::False => Token::False,
            Token::Null => Token::Null,
            Token::CastUnsignedRef => Token::CastUnsignedRef,
            Token::CastFloatRef => Token::CastFloatRef,
            Token::CastRef => Token::CastRef,
            Token::Comma => Token::Comma,
            Token::SColon => Token::SColon,
            Token::If => Token::If,
            Token::Then => Token::Then,
            Token::Else => Token::Else,
            Token::For => Token::For,
            Token::Execute => Token::Execute,
            Token::Repeat => Token::Repeat,
            Token::While => Token::While,
            Token::Until => Token::Until,
            Token::BlockEnd => Token::BlockEnd,
            Token::FloorStart => Token::FloorStart,
            Token::FloorEnd => Token::FloorEnd,
            Token::LParen => Token::LParen,
            Token::RParen => Token::RParen,
            Token::Set => Token::Set,
            Token::Add => Token::Add,
            Token::Subtract => Token::Subtract,
            Token::Multiply => Token::Multiply,
            Token::Divide => Token::Divide,
            Token::Mod => Token::Mod,
            Token::Not => Token::Not,
            Token::And => Token::And,
            Token::Or => Token::Or,
            Token::Equal => Token::Equal,
            Token::NotEqual => Token::NotEqual,
            Token::LessThan => Token::LessThan,
            Token::GreaterThan => Token::GreaterThan,
            Token::LessThanEqual => Token::LessThanEqual,
            Token::GreaterThanEqual => Token::GreaterThanEqual,
            Token::Illegal => Token::Illegal,
            Token::EOF => Token::EOF,
        }
    }
}

fn is_valid_romanian_character(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | 'ă' | 'î' | 'ț' | 'â' | 'ș' | 'Ă' | 'Î' | 'Ț' | 'Â' | 'Ș' => true,
        _ => false
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Identifier(x) => write!(f, "Identifier({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::Float(x) => write!(f, "Float({})", x),
            Token::String(x) => write!(f, "String({})", x),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Null => write!(f, "Null"),

            Token::CastUnsignedRef => write!(f, "CastUnsignedRef"),
            Token::CastFloatRef => write!(f, "CastFloatRef"),
            Token::CastRef => write!(f, "CastRef"),

            Token::Comma => write!(f, "Comma"),
            Token::SColon => write!(f, "SColon"),

            Token::If => write!(f, "If"),
            Token::Then => write!(f, "Then"),
            Token::Else => write!(f, "Else"),
            Token::For => write!(f, "For"),
            Token::Execute => write!(f, "Execute"),
            Token::Repeat => write!(f, "Repeat"),
            Token::While => write!(f, "While"),
            Token::Until => write!(f, "Until"),

            Token::BlockEnd => write!(f, "BlockEnd"),
            Token::FloorStart => write!(f, "FloorStart"),
            Token::FloorEnd => write!(f, "FloorEnd"),

            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),

            Token::Set => write!(f, "Set"),

            Token::Add => write!(f, "Add"),
            Token::Subtract => write!(f, "Subtract"),
            Token::Multiply => write!(f, "Multiply"),
            Token::Divide => write!(f, "Divide"),
            Token::Mod => write!(f, "Mod"),

            Token::Not => write!(f, "Not"),
            Token::And => write!(f, "And"),
            Token::Or => write!(f, "Or"),

            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::LessThanEqual => write!(f, "LessThanEqual"),
            Token::GreaterThanEqual => write!(f, "GreaterThanEqual"),

            Token::Illegal => write!(f, "Illegal"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Lexer {
    pos: usize,
    read_pos: usize,
    ch: char,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            pos: 0,
            read_pos: 0,
            ch: '\0',
            input,
        };

        lex.read_char();

        lex
    }

    pub fn next(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let mut should_skip = true;

        let tok = match self.ch {
            '"' => Token::String(self.read_string('"')),
            '\'' => Token::String(self.read_string('\'')),
            ',' => Token::Comma,
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '%' => Token::Mod,
            '=' => Token::Equal,
            '≠' => Token::NotEqual,
            ';' | '\n' => Token::SColon,
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Illegal
                }
            },
            '■' => Token::BlockEnd,
            '[' => {
                if self.peek() == ']' {
                    self.read_char();
                    Token::BlockEnd
                } else {
                    Token::FloorStart
                }
            }
            ']' => Token::FloorEnd,
            '(' => {
                let rest = self.input.chars().skip(self.pos).collect::<String>();
                let rest = rest.trim();
                if rest.starts_with("(număr natural)") {
                    for _ in 1..15 {
                        self.read_char()
                    }
                    Token::CastUnsignedRef
                } else if rest.starts_with("(număr)") {
                    for _ in 1..7 {
                        self.read_char()
                    }
                    Token::CastFloatRef
                } else if rest.starts_with("(număr real)") {
                    for _ in 1..12 {
                        self.read_char()
                    }
                    Token::CastFloatRef
                } else if rest.starts_with("(ref)") {
                    for _ in 1..5 {
                        self.read_char()
                    }
                    Token::CastRef
                } else {
                    Token::LParen
                }
            },
            ')' => Token::RParen,
            '<' => {
                if self.peek() == '-' {
                    self.read_char();
                    Token::Set
                } else if self.peek() == '=' {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            },
            '←' | '🡐' | '🠐' | '🠔' | '⭠' | '🠀' | '🠠' | '🡠' | '🡨' | '' => Token::Set,
            '>' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            },
            '≤' => Token::LessThanEqual,
            '≥' => Token::GreaterThanEqual,
            'a'..='z' | 'A'..='Z' | '_' | 'ă' | 'î' | 'ț' | 'ș' | 'Ă' | 'Î' | 'Ț' | 'Ș' => {
                let ident = self.read_ident().to_lowercase();
                match ident.as_ref() {
                    "adevarat" => { return Err(anyhow!("Folosește diacitice, este `adevărat`, nu `adevarat`!")); },
                    "daca" => { return Err(anyhow!("Folosește diacitice, este `dacă`, nu `daca`!")); },
                    "executa" => { return Err(anyhow!("Folosește diacitice, este `execută`, nu `executa`!")); },
                    "repeta" => { return Err(anyhow!("Folosește diacitice, este `repetă`, nu `repeta`!")); },
                    "si" => { return Err(anyhow!("Folosește diacitice, este `și`, nu `si`!")); },
                    "cattimp" => { return Err(anyhow!("Folosește diacitice, este `câttimp`, nu `cattimp`!")); },
                    "panacand" | "pânacand" | "pânăcand" | "panăcând" | "panacând" | "pânacând" | "panăcand" => { return Err(anyhow!("Folosește diacitice, este `pânăcand`, nu `panacand`!")); },
                    "cat timp" => { return Err(anyhow!("Folosește diacitice, este `cât timp`, nu `cat timp`!")); },
                    "pana cand" | "pâna cand" | "până cand" | "pană când" | "pana când" | "pâna când" | "pană cand" => { return Err(anyhow!("Folosește diacitice, este `până cand`, nu `pana cand`!")); },
                    "citeste" => { return Err(anyhow!("Folosește diacitice, este `citește`, nu `citeste`!")); },
                    _ => ()
                }

                return Ok(match ident.as_ref() {
                    "nul" => Token::Null,
                    "adevărat" => Token::True,
                    "fals" => Token::False,
                    "dacă" => Token::If,
                    "atunci" => Token::Then,
                    "altfel" => Token::Else,
                    "pentru" => Token::For,
                    "execută" => Token::Execute,
                    "repetă" => Token::Repeat,
                    "câttimp" => Token::While,
                    "pânăcând" => Token::Until,
                    "cât timp" => Token::While,
                    "până când" => Token::Until,
                    "not" => Token::Not,
                    "și" => Token::And,
                    "sau" => Token::Or,
                    _ => Token::Identifier(ident),
                })
            },
            '0'..='9' => {
                if self.ch == '0' && self.peek().is_numeric() {
                    return Err(anyhow!("Invalid number literal"));
                }

                let num = self.read_number();
                should_skip = false;

                if num.contains('.') {
                    if let Ok(f) = num.parse::<f64>() {
                        Token::Float(f)
                    } else {
                        return Err(anyhow!("Invalid floating-point literal"));
                    }
                } else {
                    if let Ok(i) = num.parse::<i64>() {
                        Token::Int(i)
                    } else {
                        return Err(anyhow!("Invalid integer literal"));
                    }
                }
            },
            '\0' => Token::EOF,
            _ => unreachable!("Caracter invalid: {}", self.ch),
        };

        if should_skip {
            self.read_char();
        }
        return Ok(tok);
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.pos;
        while self.ch.is_numeric() || self.ch == '.' {
            self.read_char();
        }
        self.input.chars().skip(start_pos).take(self.pos - start_pos).collect()
    }

    fn read_ident(&mut self) -> String {
        let pos = self.pos;
        while is_valid_romanian_character(self.ch) || self.ch == '_' {
            self.read_char()
        }
        let mut lol = self.input.chars().skip(pos).take(self.pos - pos).collect::<String>();
        match lol.as_str() {
            "până" | "pâna" | "pană" | "pana" => {
                self.skip_whitespace();
                let cand = self.read_ident();
                if matches!(cand.as_str(), "cand" | "când") {
                    lol.push(' ');
                    lol.push_str(cand.as_str());
                }
            }
            "cat" | "cât" => {
                self.skip_whitespace();
                let timp = self.read_ident();
                if timp == "timp" {
                    lol.push(' ');
                    lol.push_str(timp.as_str());
                }
            }
            _ => ()
        }
        lol
    }

    fn read_string(&mut self, ch: char) -> String {
        let position = self.pos + 1;
        loop {
            self.read_char();
            if self.ch == '\\' && self.ch == ch {
                self.read_char();
                self.read_char();
            }
            if self.ch == ch || self.ch == '\0' {
                break;
            }
        }
        return self.input.chars().skip(position).take(self.pos - position).collect::<String>().replace("\\n", "\n").replace("\\r", "\r").replace("\\t", "\t").replace("\\e", "\x1b");
    }

    fn skip_whitespace(&mut self) {
        while (self.ch.is_whitespace() && self.ch != '\n') || matches!(self.ch, '│' | '└' | '┌') {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.chars().count() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek(&mut self) -> char {
        if self.read_pos >= self.input.chars().count() {
            '\0'
        } else {
            self.input.chars().nth(self.read_pos).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn lex_basic() -> Result<()> {
        let input = String::from(",\n+ - / * %= ≠ != <- <= < >= > ≤ ≥ [] ][");
        let mut lex = Lexer::new(input);

        let tokens = vec![
            Token::Comma,
            Token::Add,
            Token::Subtract,
            Token::Divide,
            Token::Multiply,
            Token::Mod,
            Token::Equal,
            Token::NotEqual,
            Token::NotEqual,
            Token::Set,
            Token::LessThanEqual,
            Token::LessThan,
            Token::GreaterThanEqual,
            Token::GreaterThan,
            Token::LessThanEqual,
            Token::GreaterThanEqual,
            Token::BlockEnd,
            Token::FloorEnd,
            Token::FloorStart,
            Token::EOF,
        ];

        for token in tokens {
            let tok = lex.next()?;
            assert_eq!(tok, token);
        }

        Ok(())
    }

    #[test]
    fn lex_ident_num() -> Result<()> {
        let input = String::from("banane mâncare țigan înalt ăla \
                                  123 123,456.690");
        let mut lex = Lexer::new(input);

        let tokens = vec![
            Token::Identifier(String::from("banane")),
            Token::Identifier(String::from("mâncare")),
            Token::Identifier(String::from("țigan")),
            Token::Identifier(String::from("înalt")),
            Token::Identifier(String::from("ăla")),
            Token::Int(123),
            Token::Float(123.456_69),
            Token::EOF,
        ];

        for token in tokens {
            let tok = lex.next()?;
            assert_eq!(tok, token);
        }

        Ok(())
    }

    #[test]
    fn lex_keyword() -> Result<()> {
        let input = String::from("dacă atunci altfel pentru execută repetă câttimp pânăcând not și sau");
        let mut lex = Lexer::new(input);

        let tokens = vec![
            Token::If,
            Token::Then,
            Token::Else,
            Token::For,
            Token::Execute,
            Token::Repeat,
            Token::While,
            Token::Until,
            Token::Not,
            Token::And,
            Token::Or,
            Token::EOF,
        ];

        for token in tokens {
            let tok = lex.next()?;
            assert_eq!(tok, token);
        }

        Ok(())
    }
}

