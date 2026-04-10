use crate::{d_log, debug};
use std::{path::PathBuf, process::exit};

#[derive(Clone, Debug)]
pub struct RunMode {
    pub verbose: bool,
    pub list: bool,
    pub normal: bool,
    pub minimal: bool,
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
        let mut add_g = false;
        let mut add_f = false;
        let mut add_s = false;
        let mut is_sadd = true;

        let mut add_file = false;
        let mut is_fadd = true;

        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars() {
                    match ch {
                        '-' => continue,
                        'h' => {
                            print_help();
                            exit(0);
                        }
                        'l' => self.runmod.list = true,
                        'g' => {
                            add_g = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_f, &mut add_file);
                        }
                        'a' => {
                            add_f = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_g, &mut add_file);
                        }
                        's' => {
                            add_s = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_file);
                        }
                        'f' => {
                            add_file = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s);
                        }
                        'm' => self.runmod.minimal = true,
                        'v' => {
                            self.runmod.verbose = true;
                            debug::set_debug(self.runmod.verbose);
                        }
                        _ => {
                            println!("option {ch} not found!");
                            exit(1);
                        }
                    }
                }
                if arg.starts_with("--") {
                    match arg.as_str() {
                        "--help" => {
                            print_help();
                            exit(0);
                        }
                        "--list" => {
                            self.runmod.list = true;
                        }
                        "--add" => {
                            add_f = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_g, &mut add_file);
                        }
                        "--get" => {
                            add_g = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_f, &mut add_file);
                        }
                        "--set-weight" => {
                            add_s = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_file);
                        }
                        "--s-file" => {
                            add_file = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s);
                        }
                        "--minimal" => self.runmod.minimal = true,
                        "--verbose" => {
                            self.runmod.verbose = true;
                            debug::set_debug(self.runmod.verbose);
                        }
                        _ => {
                            println!("option arg {arg} not found");
                            exit(1);
                        }
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
                d_log!("-> adding file path : {arg}");
                is_fadd = false;
                self.file = PathBuf::from(arg);
            } else if add_g {
                d_log!("-> adding item {arg} to get list");
                self.data_tg.push(arg.to_string());
            } else if add_f {
                d_log!("-> add item {arg} to add list");
                self.data_tf.push(arg.to_string());
            } else {
                d_log!("-> adding item {arg} to normal list");
                self.data_ta.push(arg.to_string());
            }
        }
    }
    pub fn new() -> Self {
        Self {
            data_tg: Vec::new(),
            data_ta: Vec::new(),
            data_tf: Vec::new(),
            data_s: 50.0,
            runmod: RunMode::new(),
            file: PathBuf::from("/home/wassim/foo/cal/data/data.yaml"),
        }
    }
}
impl RunMode {
    pub fn new() -> Self {
        Self {
            list: false,
            minimal: false,
            normal: true,
            verbose: false,
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
