use crate::{d_log, debug};
use std::{path::PathBuf, process::exit};

#[derive(Clone, Debug)]
pub struct RunMode {
    pub verbose: bool,
    pub list: bool,
    pub normal: bool,
}

#[derive(Clone, Debug)]
pub struct FlData {
    pub data_tg: Vec<String>,
    pub data_ta: Vec<String>,
    pub data_tf: Vec<String>,
    pub data_s: f64,
    pub runmod: RunMode,
    pub file: PathBuf,
}
fn set_false(b1: &mut bool, b2: &mut bool, b3: &mut bool) {
    *b1 = false;
    *b2 = false;
    *b3 = false;
}
impl FlData {
    pub fn parse_args(&mut self, args: &Vec<String>) {
        for arg in args.iter() {
            self.runmod.verbose =
                (arg.starts_with("-") && arg.contains("v")) || arg.eq("--verbose");
            if self.runmod.verbose {
                break;
            }
        }
        debug::set_debug(self.runmod.verbose);

        let mut add_g = false;
        let mut add_f = false;
        let mut add_s = false;
        let mut is_sadd = true;

        let mut add_file = false;
        let mut is_fadd = true;

        d_log!("==> handling args");

        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars() {
                    if ch.eq(&'-') {
                        continue;
                    }
                    if ch.eq(&'h') {
                        print_help();
                        exit(0);
                    } else if ch.eq(&'l') {
                        d_log!("-> flag 'l' found");
                        self.runmod.list = true;
                    } else if ch.eq(&'g') {
                        d_log!("-> flag 'g' found");
                        add_g = true;
                        self.runmod.normal = true;
                        set_false(&mut add_s, &mut add_f, &mut add_file);
                    } else if ch.eq(&'a') {
                        d_log!("-> flag 'a' found");
                        add_f = true;
                        self.runmod.normal = true;
                        set_false(&mut add_s, &mut add_g, &mut add_file);
                    } else if ch.eq(&'s') {
                        d_log!("-> flag 's' found");
                        add_s = true;
                        self.runmod.normal = true;
                        set_false(&mut add_g, &mut add_f, &mut add_file);
                    } else if ch.eq(&'f') {
                        d_log!("-> flag 'f' found");
                        add_file = true;
                        self.runmod.normal = true;
                        set_false(&mut add_g, &mut add_f, &mut add_s);
                    } else if ch.eq(&'v') {
                    } else {
                        println!("option ch : {ch} not found");
                        exit(1);
                    }
                }
                if arg.starts_with("--") {
                    if arg.eq("--help") {
                        print_help();
                        exit(0);
                    } else if arg.eq("--list") {
                        d_log!("-> flag 'list' found");
                        self.runmod.list = true;
                    } else if arg.eq("--add") {
                        d_log!("-> flag 'add' found");
                        add_f = true;
                        self.runmod.normal = true;
                        set_false(&mut add_s, &mut add_g, &mut add_file);
                    } else if arg.eq("--get") {
                        d_log!("-> flag 'get' found");
                        add_g = true;
                        self.runmod.normal = true;
                        set_false(&mut add_s, &mut add_f, &mut add_file);
                    } else if arg.eq("--set-weight") {
                        d_log!("-> flag 'set-weight' found");
                        add_s = true;
                        self.runmod.normal = true;
                        set_false(&mut add_g, &mut add_f, &mut add_file);
                    } else if arg.eq("--s-file") {
                        d_log!("flag 's-file' found");
                        add_file = true;
                        self.runmod.normal = true;
                        set_false(&mut add_g, &mut add_f, &mut add_s);
                    } else if arg.eq("--verbose") {
                    } else {
                        println!("option arg {arg} not found");
                        exit(1);
                    }
                }
                continue;
            }

            if add_s && is_sadd {
                d_log!("adding weight {arg} - supposed to be once");
                is_sadd = false;
                self.data_s = match arg.parse() {
                    Ok(val) => val,
                    Err(err) => {
                        dbg!(err);
                        50.0
                    }
                }
            } else if add_file && is_fadd {
                d_log!("adding file path : {arg}");
                is_fadd = false;
                self.file = PathBuf::from(arg);
            } else if add_g {
                d_log!("adding item {arg} to get list");
                self.data_tg.push(arg.to_string());
            } else if add_f {
                d_log!("add item {arg} to add list");
                self.data_tf.push(arg.to_string());
            } else {
                d_log!("adding item {arg} to normal list");
                self.data_ta.push(arg.to_string());
            }
        }
    }
}

fn print_help() {
    println!(
        "        Ncal - Nutrition Calculator
        Usage: ncal [OPTIONS] [args]

Options:
    -h, --help         Show this help message
    -v, --verbose      Enable verbose output
    -s, --set-weight   Set the weight to
    -a, --add          Remember the input
    -g, --get          Get nutrition of the input
    -l, --list         List all food"
    );
}
