mod objects;

use objects::Nut;

use crate::objects::build_objects;
use evalexpr::*;
use std::env::{self};
use std::io::{Write, stdin, stdout};
use std::process::exit;

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
    let num = eval_int(number).expect("error eval") as f64;
    (text, num)
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
        let mut n: i64 = 0;
        let mut neg = false;

        if s.starts_with('-') {
            s.remove(0);
            neg = true;
        }

        if s.starts_with('+') {
            s.remove(0);
            neg = false;
        }

        if s.as_bytes()[0].is_ascii_digit() || neg {
            for b in s.bytes() {
                if b.is_ascii_digit() {
                    n = n * 10 + (b - b'0') as i64;
                    continue;
                }
                break;
            }
            if neg {
                n = -n;
            }
            snut.cal += n as f64;
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
}
