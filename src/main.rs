#![feature(let_chains)]

use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use std::{path::Path, rc::Rc, cell::RefCell};
use std::env;
use crossterm::cursor;
use home::home_dir;
use interpreter::{Environment, eval};
use rustyline::{Editor, history::DefaultHistory, validate::Validator, hint::{Hinter, HistoryHinter}, Context, highlight::Highlighter, Helper, completion::{Completer, Pair}, error::ReadlineError, config::Configurer};

use lexer::Lexer;
use parser::Parser;

use crate::interpreter::Object;

mod lexer;
mod parser;
mod interpreter;

struct CustomHelper {
    hinter: HistoryHinter,
}

impl Validator for CustomHelper {}
impl Highlighter for CustomHelper {}
impl Helper for CustomHelper {}
impl Completer for CustomHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut completions = Vec::new();
        let word_start = match line[..pos].rfind(|c: char| c.is_whitespace() || c == '(' || c == ')') {
            Some(index) => index + 1,
            None => 0,
        };
        let word = &line[word_start..pos];

        if !word.is_empty() {
            for i in vec!["dacă", "până când", "pânăcând", "cât timp", "câttimp", "atunci", "altfel", "pentru", "execută", "scrie", "citește"] {
                if i.starts_with(word) {
                    completions.push(Pair {
                        display: i.to_string(),
                        replacement: i.to_string(),
                    });
                }
            }
        }

        Ok((word_start, completions))
    }
}

impl Hinter for CustomHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

type CustomEditor = Editor<CustomHelper, DefaultHistory>;

fn main() {
    let envb = Rc::new(RefCell::new(Environment::new()));

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let mut filep = args[1].clone();
        let mut file = Path::new(&filep);
        if !file.exists() {
            filep.push_str(".ro");
            file = Path::new(&filep);
            if !file.exists() {
                std::io::stderr().write(format!("Nu există fișierul `{}`!\n", filep).as_bytes()).unwrap();
                exit(1);
            }
        }
        let input = read_to_string(file).unwrap();

        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let root = parser.parse();
        //root.print(String::new(), true);

        if parser.errors.len() > 0 {
            println!("Erori găsite:");
            for (pos, msg) in parser.errors.iter().enumerate() {
                println!(" {}: {}", pos+1, msg);
            }
            return;
        }

        eval(root, Some(Rc::clone(&envb))).unwrap();
        return;
    }

    let mut rl_hist = home_dir().unwrap();
    rl_hist.push(Path::new(".rolang_history"));
    let rl_hist = rl_hist.to_str().unwrap();

    let mut rl = CustomEditor::new().unwrap();
    if rl.load_history(rl_hist).is_err() {
        println!("Istoric nou creat.");
    }

    rl.set_helper(Some(CustomHelper { hinter: HistoryHinter {  }}));
    rl.set_completion_type(rustyline::CompletionType::Circular);

    loop {
        let readline = rl.readline("> ");
        if readline.is_err() {
            break;
        }
        let mut input = readline.unwrap();

        if input.trim().is_empty() {
            continue;
        }

        rl.add_history_entry(input.clone()).unwrap();

        input.push('\n');

        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let root = parser.parse();
        //root.print(String::new(), true);

        if parser.errors.len() > 0 {
            println!("Erori găsite:");
            for (pos, msg) in parser.errors.iter().enumerate() {
                println!(" {}: {}", pos+1, msg);
            }
            continue;
        }

        let res = eval(root, Some(Rc::clone(&envb)));
        match cursor::position() {
            Ok((x, _)) => if x != 0 {
                println!("\x1b[7m%\x1b[0m");
            },
            Err(_) => ()
        };
        match res {
            Err(e) => println!("Eroare ROLang: {}", e),
            Ok(res) => if res != Object::Null {
                println!("Rezultat: {}", res);
            }
        }
    }

    rl.save_history(rl_hist).unwrap();
}
