use rustyline::{Editor, history::DefaultHistory, validate::Validator, hint::{Hinter, HistoryHinter}, Context, highlight::Highlighter, Helper, completion::{Completer, Pair}, error::ReadlineError, config::Configurer};
use crossterm::cursor;
use home::home_dir;
use std::{rc::Rc, cell::RefCell};
use std::path::Path;
use anyhow::{anyhow, Result};

use crate::interpreter::{Environment, eval};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Object;

pub struct CustomHelper {
    pub hinter: HistoryHinter,
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

pub type RoLangReadlineEditor = Editor<CustomHelper, DefaultHistory>;

pub fn exec(input: &str) -> Result<Object> {
    let envb = Rc::new(RefCell::new(Environment::new()));

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let root = parser.parse();
    //root.print(String::new(), true);

    if parser.errors.len() > 0 {
        println!("Erori găsite:");
        for (pos, msg) in parser.errors.iter().enumerate() {
            println!(" {}: {}", pos+1, msg);
        }
        return Err(anyhow!("Există erori."));
    }

    eval(root, Some(Rc::clone(&envb)))
}

pub fn repl() {
    let mut rl_hist = home_dir().unwrap();
    rl_hist.push(Path::new(".rolang_history"));
    let rl_hist = rl_hist.to_str().unwrap();

    let mut rl = RoLangReadlineEditor::new().unwrap();
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

        let res = exec(&input);
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
