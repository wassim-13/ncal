use crate::{d_log, debug, run};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    o_path: PathBuf,
}

fn set_o_file<P: AsRef<Path>>(path: P) {
    let temp_obj = Config {
        o_path: PathBuf::from(path.as_ref()),
    };
    let c_path = env::home_dir().unwrap().join(".config/ncal/config.json");
    let content: String = serde_json::to_string(&temp_obj).unwrap_or_default();
    fs::write(c_path, content).unwrap();
}

fn cool_function() -> Config {
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());
    let path = home.join(".config/ncal/config.json");

    let ob_path = home.join(".local/share/ncal/objects.yaml");

    if !ob_path.exists()
        && let Some(o_parent) = ob_path.parent()
    {
        fs::create_dir_all(o_parent).expect("Failed to create parent directories");
        fs::write(&ob_path, "").unwrap_or_default();
    }

    let mut config = Config { o_path: ob_path };

    if path.exists() {
        let contents = fs::read_to_string(&path).unwrap_or_default();
        let obj: Option<Config> = serde_json::from_str(&contents).ok();
        if let Some(conf) = obj {
            config = conf;
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directories");
    }

    let content = serde_json::to_string(&config).unwrap_or_default();

    fs::write(path, &content).expect("Failed to write default data to file");
    config
}

#[derive(Clone, Debug)]
pub struct RunMode {
    pub verbose: bool,
    pub list: bool,
    pub normal: bool,
    pub needs: bool,
    pub minimal: bool,
    pub insert: bool,
    pub clear: bool,
}

#[derive(Clone, Debug)]
pub struct FlData {
    pub data_tg: Vec<String>,
    pub data_ta: Vec<String>,
    pub data_tf: Vec<String>,
    pub data_s: f64,
    pub runmod: RunMode,
    pub t_file: PathBuf,
    pub o_file: PathBuf,
}
fn set_false(b1: &mut bool, b2: &mut bool, b3: &mut bool, b4: &mut bool) {
    *b1 = false;
    *b2 = false;
    *b3 = false;
    *b4 = false;
}
impl FlData {
    pub fn parse_args(&mut self, args: &Vec<String>) {
        let mut add_g = false;
        let mut add_f = false;
        let mut add_s = false;
        let mut is_sadd = true;

        let mut add_file = false;
        let mut is_fadd = true;

        let mut add_ofile = false;
        let mut is_ofadd = true;

        for arg in args {
            d_log!("arg {arg}");
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        '-' => break,
                        'h' => {
                            d_log!("found option h");
                            print_help();
                            exit(0);
                        }
                        'l' => {
                            d_log!("found option l");
                            self.runmod.list = true;
                            self.runmod.normal = false
                        }
                        'g' => {
                            d_log!("found option g");
                            add_g = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_f, &mut add_file, &mut add_ofile);
                        }
                        'a' => {
                            d_log!("found option a");
                            add_f = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_g, &mut add_file, &mut add_ofile);
                        }
                        'c' => {
                            d_log!("found option c");
                            self.runmod.clear = true;
                        }
                        'w' => {
                            d_log!("found option w");
                            add_s = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_file, &mut add_ofile);
                        }
                        'f' => {
                            d_log!("found option f");
                            add_file = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s, &mut add_ofile);
                        }
                        'o' => {
                            d_log!("found option o");
                            add_ofile = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s, &mut add_file);
                        }
                        'm' => {
                            d_log!("found option m");
                            self.runmod.minimal = true;
                        }
                        'n' => {
                            d_log!("found option n");
                            self.runmod.needs = true;
                        }
                        'v' => {
                            d_log!("found option v");
                            self.runmod.verbose = true;
                            debug::set_debug(self.runmod.verbose);
                        }
                        'V' => {
                            println!("ncal v{}", env!("CARGO_PKG_VERSION"));
                            exit(0);
                        }
                        'i' => {
                            d_log!("found option i");
                            println!("adding items will cost unecessarly performance!");
                            println!("add items by your self bro");
                            println!("here' the file to add stuff $proj_dir/data/objects.yaml");
                            println!("see it's just yaml to make your life easier ;)");
                        }
                        'I' => {
                            d_log!("found option I");
                            self.runmod.insert = true;
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
                            d_log!("found option {arg}");
                            print_help();
                            exit(0);
                        }
                        "--list" => {
                            d_log!("found option {arg}");
                            self.runmod.list = true;
                            self.runmod.normal = false;
                        }
                        "--clear" => {
                            d_log!("found option {arg}");
                            self.runmod.clear = true;
                        }
                        "--add" => {
                            d_log!("found option {arg}");
                            add_f = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_g, &mut add_file, &mut add_ofile);
                        }
                        "--get" => {
                            d_log!("found option {arg}");
                            add_g = true;
                            self.runmod.normal = true;
                            set_false(&mut add_s, &mut add_f, &mut add_file, &mut add_ofile);
                        }
                        "--set-weight" => {
                            d_log!("found option {arg}");
                            add_s = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_file, &mut add_ofile);
                        }
                        "--insert" => {
                            d_log!("found option {arg}");
                            println!("adding items will cost unecessarly performance!");
                            println!("add items by your self bro");
                            println!("here' the file to add stuff $proj_dir/data/objects.yaml");
                            println!("see it's just yaml to make your life easier ;)");
                        }
                        "--force-insert" => {
                            d_log!("found option {arg}");
                            self.runmod.insert = true;
                        }
                        "--s-file" => {
                            d_log!("found option {arg}");
                            add_file = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s, &mut add_ofile);
                        }
                        "--o-file" => {
                            d_log!("found option {arg}");
                            add_file = true;
                            self.runmod.normal = true;
                            set_false(&mut add_g, &mut add_f, &mut add_s, &mut add_ofile);
                        }
                        "--minimal" => {
                            d_log!("found option {arg}");
                            self.runmod.minimal = true
                        }
                        "--needs" => {
                            d_log!("found option {arg}");
                            self.runmod.needs = true;
                        }
                        "--verbose" => {
                            d_log!("found option {arg}");
                            self.runmod.verbose = true;
                            debug::set_debug(self.runmod.verbose);
                        }
                        "--version" => {
                            println!("ncal v{}", env!("CARGO_PKG_VERSION"));
                            exit(0);
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
                d_log!("-> adding file path of total : {arg}");
                if !run::check_file(arg) {
                    println!("file {arg} does not exist! skipping..");
                    continue;
                };
                is_fadd = false;
                self.t_file = PathBuf::from(arg);
            } else if add_ofile && is_ofadd {
                d_log!("-> adding file path of objects : {arg}");
                if !run::check_file(arg) {
                    println!("file {arg} does not exist! skipping..");
                    continue;
                };
                is_ofadd = false;
                self.o_file = PathBuf::from(&arg);
                // TODO:
                //  add the file to the json conf
                set_o_file(arg.as_str());
                //--------
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
        let default_app_dir = env::home_dir().unwrap_or_default().join(".ncal");

        Self {
            data_tg: Vec::new(),
            data_ta: Vec::new(),
            data_tf: Vec::new(),
            data_s: 70.0,
            runmod: RunMode::new(),
            t_file: default_app_dir.join("data/data.yaml"),
            o_file: cool_function().o_path,
            // TODO :
            // the default path should be loaded from json conf
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
            insert: false,
            clear: false,
            needs: false,
        }
    }
}

fn print_help() {
    println!(
        "
        Ncal - Nutrition Calculator
    
    Usage: ncal [OPTIONS] [args]

Options:
    -h, --help         Show this help message
    -V, --version      Print Version and exit
    -a, --add          Remember the input
    -c, --clear        Clear the saved total nut
    -f, --s-file       Set where to save total nut 
    -g, --get          Get nutrition of the input
    -i, --insert       Insert an item to objects 
    -l, --list         List all food
    -m, --minimal      Minimal mode
    -n, --needs        Print needs based on weight
    -o, --o-file       Set where objects are located
    -v, --verbose      Enable verbose output
    -w, --set-weight   Set the weight
"
    );
}
