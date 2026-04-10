use crate::d_log;
use crate::flags::FlData;
use crate::objects::{self, Nut};
use crate::parse::{eval_num, parse_input};
use crate::printing::{Color, progress_bar};

use std::path::{Path, PathBuf};
use std::{collections::HashMap, env, fs::OpenOptions, process};

#[derive(Clone, Debug)]
pub struct RunC {
    data: FlData,
    args: Vec<String>,
    tnut: Nut,
    pnut: Nut,
    tliter: f64,
    objects: HashMap<&'static str, Nut>,
}

enum DM {
    Fd,
    Nd,
}

impl RunC {
    pub fn new() -> Self {
        Self {
            data: FlData::new(),
            args: Vec::new(),
            tnut: Nut::new(),
            pnut: Nut::new(),
            tliter: 3.0,
            objects: HashMap::new(),
        }
    }
    pub fn init(&mut self) {
        self.objects = objects::build_objects();
        self.args = env::args().collect();
        self.args.remove(0);
        self.data.parse_args(&self.args);
        d_log!("-> parsing args, done");
        self.args.clear();
        d_log!("-> clearing args, done");
        if !check_file(&self.data.file) {
            self.data.file = PathBuf::from("/home/wassim/foo/cal/data/data.txt");
        }
        d_log!(
            "-> setting filepath, done\n        filepath : {}",
            self.data.file.display()
        );
        self.tnut = Nut {
            cal: 0.0,
            carb: 0.0,
            prot: 0.0,
            fiber: 0.0,
            fat: 0.0,
        };
        d_log!("-> setting total nut, done");
        let weight = self.data.data_s;
        let prtn = 2.0 * weight;
        let crbs = 7.0 * weight;
        let fts = 1.5 * weight;
        let cals = 4.0 * prtn + 4.0 * crbs + 9.0 * fts;
        let fbrs = 0.014 * cals;

        self.pnut = Nut {
            cal: cals,
            carb: crbs,
            prot: prtn,
            fiber: fbrs,
            fat: fts,
        };
        d_log!(
            "-> setting perfect nut, done\n      perfect nuts : {:#?}",
            self.pnut
        );
        self.tliter = 2.0;
    }
    pub fn run(mut self) {
        // alias vars
        let runmod = &self.data.runmod;
        let data = &self.data;

        if runmod.normal && runmod.list {
            println!("incompatible options `list` and `normal`");
            println!("however heres the list");
            self.list_all();
            process::exit(0);
        }
        if !data.data_tg.is_empty() {
            self.run_as_getn();
        }

        let b1 = !data.data_ta.is_empty();
        let b2 = !data.data_tf.is_empty();

        if b1 {
            self.run_as_normal(DM::Nd);
        }
        if b2 {
            self.run_as_addn();
        }
        self.print_water();
    }
    fn run_as_getn(&self) {
        d_log!("->> running as get!");
        for s in self.data.data_tg.iter() {
            println!("\n=> {s}\n");

            match self.objects.get(s.as_str()) {
                Some(obj) => obj.print(),
                None => println!("object {s} not found!"),
            }
        }
    }
    fn run_as_normal(&mut self, mode: DM) {
        d_log!("->> running as normal!");
        let data = match mode {
            DM::Fd => &mut self.data.data_tf,
            DM::Nd => &mut self.data.data_ta,
        };

        for s in data.iter_mut() {
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
                let val = eval_num(s.as_str());
                self.tnut.cal += val;
                d_log!("-> adding value {val} to total nut cal, done");
                continue;
            } else if (temp_b1 || temp_b2 || neg) && liter {
                if s.ends_with("m") {
                    s.pop();
                    let val = eval_num(s.as_str()) / 1000.0;
                    d_log!("-> adding value {val} to total liter, done");
                    self.tliter += val;
                    continue;
                }
                let val = eval_num(s.as_str()) / 1000.0;
                d_log!("-> adding value {val} to total liter, done");
                self.tliter += val;

                continue;
            }

            let (tname, val) = parse_input(s.as_str());

            if let Some(obj) = self.objects.get_mut(tname) {
                obj.scal(val);
                self.tnut.add(obj);
                d_log!("added obj values to total nut");
            } else {
                println!("Object {tname} not found");
            }
        }
        d_log!("heres total nut : {:#?}", self.tnut);
        println!("=> total nuts");
        self.tnut.print();
        println!("=> left nuts");
        self.tnut.printb(&self.pnut);
    }
    fn run_as_addn(&mut self) {
        d_log!("->> running as add");
        self.tnut = objects::get_nut_from_file(&self.data.file);
        d_log!("-> total nut before normal run : {:#?}", self.tnut);
        self.run_as_normal(DM::Fd);
        d_log!("-> total nut after normal run: {:#?}", self.tnut);
        objects::store_nut_to_file(&self.data.file, &self.tnut).unwrap_or_default();
    }
    fn list_all(self) {
        for key in self.objects.keys() {
            println!("{key}");
        }
    }
    fn print_water(&self) {
        progress_bar(self.tliter, 2.0, 26, Color::Blue);
    }
}

fn check_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    if !path.exists() {
        println!("file does not exitst");
        return false;
    }
    OpenOptions::new().read(true).write(true).open(path).is_ok()
}
