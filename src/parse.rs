use evalexpr::*;
use std::io::{Write, stdin, stdout};

use crate::d_log;

pub enum Nwc<'a> {
    C(f64),
    W(f64),
    N(&'a str, f64),
}

fn eval_num(expr: &str) -> f64 {
    d_log!("==> evaluating expr {expr}");
    match eval_float(expr) {
        Ok(val) => {
            d_log!("-> expression = {val}");
            val
        }
        Err(err) => match err {
            EvalexprError::ExpectedFloat { actual: _ } => match eval_int(expr) {
                Ok(val) => {
                    d_log!("-> expression = {val}");
                    val as f64
                }
                Err(_) => {
                    d_log!("-> expression = 0.0");
                    0.0
                }
            },
            _ => {
                d_log!("-> expression = 0.0");
                0.0
            }
        },
    }
}

fn parse_input(s: &str) -> (&str, f64) {
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

pub fn get_w_add(s: &mut String) -> Nwc<'_> {
    d_log!("-> handling {s} expression");
    let mut neg = false;
    let mut liter = false;
    if s.starts_with('-') {
        s.remove(0);
        neg = true;
    }

    if s.starts_with('+') {
        s.remove(0);
        neg = true;
    }

    if s.ends_with("l") {
        s.pop();
        liter = true;
    }

    let temp_b1 = s.starts_with("(");
    let temp_b2 = s.as_bytes()[0].is_ascii_digit();
    if (temp_b2 || neg || temp_b1) && !liter {
        return Nwc::C(eval_num(s.as_str()));
    } else if (temp_b1 || temp_b2 || neg) && liter {
        if s.ends_with("m") {
            s.pop();
            return Nwc::W(eval_num(s.as_str()) / 1000.0);
        }
        return Nwc::W(eval_num(s.as_str()));
    }

    let (tname, val) = parse_input(s.as_str());
    Nwc::N(tname, val)
}
