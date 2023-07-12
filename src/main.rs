use std::{path::Path, rc::Rc, cell::RefCell};
use home::home_dir;
use interpreter::{Environment, eval};
use rustyline::{Editor, history::DefaultHistory, validate::Validator, hint::{Hinter, HistoryHinter}, Context, highlight::Highlighter, Helper, completion::{Completer, Pair}, error::ReadlineError, config::Configurer};

use lexer::Lexer;
use parser::Parser;

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
            completions.push(Pair {
                display: "dacă".to_string(),
                replacement: "dacă".to_string(),
            });
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
    let mut rl_hist = home_dir().unwrap();
    rl_hist.push(Path::new(".rolang_history"));
    let rl_hist = rl_hist.to_str().unwrap();

    let mut rl = CustomEditor::new().unwrap();
    if rl.load_history(rl_hist).is_err() {
        println!("New history file created.");
    }

    rl.set_helper(Some(CustomHelper { hinter: HistoryHinter {  }}));
    rl.set_completion_type(rustyline::CompletionType::Circular);

    let envb = Rc::new(RefCell::new(Environment::new()));

    loop {
        let readline = rl.readline("> ");
        if readline.is_err() {
            break;
        }
        let input = readline.unwrap();

        if input.trim().is_empty() {
            continue;
        }

        rl.add_history_entry(input.clone()).unwrap();

        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let root = parser.parse();
        //root.print(String::new(), true);

        if parser.errors.len() > 0 {
            println!("Errors found:");
            for (pos, msg) in parser.errors.iter().enumerate() {
                println!(" {}: {}", pos+1, msg);
            }
            continue;
        }

        println!("Rezultat: {}", eval(root, Some(Rc::clone(&envb))).unwrap());
    }

    rl.save_history(rl_hist).unwrap();
}
