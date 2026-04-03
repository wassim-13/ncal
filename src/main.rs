mod objects;

use objects::{Nut, getnut};

use evalexpr::*;
use std::env;
use std::time::Instant;

fn parse_input(s: &str) -> (&str, f64) {
    let (text, number) = s
        .rsplit_once('_')
        .expect("Input must contain exactly one '_'");

    if number.is_empty() {
        return (text, 0.0);
    };
    let num = eval_int(number).expect("error eval") as f64;
    (text, num)
}

fn main() {
    let start = Instant::now();

    let mut args: Vec<String> = env::args().collect();

    let mut snut = Nut {
        cal: 0.0,
        carb: 0.0,
        prot: 0.0,
        fiber: 0.0,
        fat: 0.0,
    };

    args.swap_remove(0);

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

        if let Some(mut obj) = getnut(tname) {
            obj.scal(val);
            snut.add(obj);
        } else {
            println!("Object {tname} not found");
        }
    }

    println!("-----total---------");
    snut.print();

    let weight = 50.0;
    let prtn = 2.0 * weight;
    let crbs = 7.0 * weight;
    let fts = 1.5 * weight;
    let cals = 4.0 * prtn + 4.0 * crbs + 9.0 * fts;
    let fbrs = 0.014 * cals;

    snut.scal(-1.0);
    snut.add(Nut {
        cal: cals,
        carb: crbs,
        prot: prtn,
        fiber: fbrs,
        fat: fts,
    });
    println!("\n-----left----------");
    snut.printb();

    let end = Instant::now();
    let start = end.duration_since(start).as_micros();

    println!("time took : {start} us");
}
