use evalexpr::*;
use std::io::{Write, stdin, stdout};

use crate::d_log;

pub fn eval_num(expr: &str) -> f64 {
    d_log!("==> evaluating expr {expr}");
    match eval_float(expr) {
        Ok(val) => val,
        Err(err) => match err {
            EvalexprError::ExpectedFloat { actual: _ } => match eval_int(expr) {
                Ok(val) => val as f64,
                Err(_) => 0.0,
            },
            _ => 0.0,
        },
    }
}

pub fn parse_input(s: &str) -> (&str, f64) {
    d_log!("parsing input function in parse.rs");
    let mut num = String::new();
    let (text, number) = match s.rsplit_once("_") {
        Some(val) => val,
        None => {
            print!("value for {s} ? : ");
            stdout().flush().unwrap_or_default();
            stdin().read_line(&mut num).expect("failed to read number!");
            (s, num.as_str())
        }
    };

    if number.is_empty() {
        return (text, 0.0);
    };
    let number = eval_num(number);
    (text, number)
}
