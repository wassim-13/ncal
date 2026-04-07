mod objects;

use objects::Nut;

use crate::objects::build_objects;
use evalexpr::*;
use std::env::{self};
use std::io::{LineWriter, Write, stdin, stdout};
use std::process::exit;

fn eval_num(expr: &str) -> f64 {
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

fn parse_input(s: &str) -> (&str, f64) {
    let mut num = String::new();
    let (text, number) = match s.rsplit_once("_") {
        Some(val) => val,
        None => {
            print!("value for {s} ? : ");
            stdout().flush().unwrap();
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

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut snut = Nut {
        cal: 0.0,
        carb: 0.0,
        prot: 0.0,
        fiber: 0.0,
        fat: 0.0,
    };

    let mut tliter = 0.0;

    let mut weight = 50.0;

    if args.len() > 2 && args[1].eq("-s") {
        if args.len() > 2 {
            weight = args[2].trim().parse().expect("failed to read weight");
        }

        args.swap_remove(1);
        match args.len() {
            2 => args.swap_remove(1),
            _ => args.swap_remove(2),
        };
    }

    args.swap_remove(0);

    if args.is_empty() {
        exit(0);
    }

    let mut objs = build_objects();

    if args.len() > 1 && args[1].trim().eq("-g") {
        args.swap_remove(1);
        for s in args {
            println!("\n=> {s}\n");

            match objs.get(s.as_str()) {
                Some(obj) => obj.print(),
                None => println!("object {s} not found!"),
            }
        }
        exit(0);
    }

    let b1 = args.len() == 1 && args[0].trim().eq("-l");
    let b2 = args.len() > 1 && args[1].trim().eq("-l");
    if b1 || b2 {
        for key in objs.keys() {
            println!("{key}");
        }
        exit(0);
    }

    for mut s in args {
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
            snut.cal += eval_num(s.as_str());
            continue;
        } else if (temp_b1 || temp_b2 || neg) && liter {
            if s.ends_with("m") {
                s.pop();
                tliter += eval_num(s.as_str()) / 1000.0;
                continue;
            }
            tliter += eval_num(s.as_str());
            continue;
        }

        let (tname, val) = parse_input(s.as_str());

        if let Some(obj) = objs.get_mut(tname) {
            obj.scal(val);
            snut.add(obj);
        } else {
            println!("Object {tname} not found");
        }
    }

    println!("-----total---------");
    snut.print();

    let prtn = 2.0 * weight;
    let crbs = 7.0 * weight;
    let fts = 1.5 * weight;
    let cals = 4.0 * prtn + 4.0 * crbs + 9.0 * fts;
    let fbrs = 0.014 * cals;

    snut.scal(-1.0);
    snut.add(&Nut {
        cal: cals,
        carb: crbs,
        prot: prtn,
        fiber: fbrs,
        fat: fts,
    });

    for th in &mut snut {
        if *th < 0.0 {
            *th = 0.0;
        }
    }

    println!("\n-----left----------");
    snut.printb();
    println!("\n-----water----------");
    println!("{}", progress_bar(tliter, 2.0));
}

fn progress_bar(current: f64, total: f64) -> String {
    let width = 25;
    let ratio = if total <= 0.0 {
        0.0
    } else {
        (current / total).clamp(0.0, 1.0)
    };

    let filled = (ratio * width as f64).round() as usize;
    let empty = width - filled;

    let filled_part = format!("\x1b[36m{}\x1b[0m", "■".repeat(filled));
    let empty_part = "□".repeat(empty);

    format!("[{}{}] {:.1}%", filled_part, empty_part, ratio * 100.0)
}
