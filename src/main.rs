mod objects;

use objects::{getnut, Nut};

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

        if s.bytes().nth(0).unwrap().is_ascii_digit() || neg {
            for b in s.bytes() {
                if b.is_ascii_digit() {
                    n = n * 10 + (b - b'0') as i64;
                } else {
                    break;
                }
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

    snut.scal(-1.0);
    snut.add(Nut {
        cal: 3000.0,
        carb: 450.0,
        prot: 110.0,
        fiber: 30.0,
        fat: 80.0,
    });
    println!("\n-----left----------");
    snut.print();

    let end = Instant::now();
    let start = end.duration_since(start).as_micros();

    println!("time took : {start} us");
}
