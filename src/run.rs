use crate::d_log;
use crate::flags::FlData;
use crate::objects::{self, Nut};
use crate::parse::{Nwc, get_w_add};
use crate::printing::{Color, progress_bar};

use std::fs;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, env, fs::OpenOptions, process};

#[derive(Clone, Debug)]
struct Objects {
    objects: HashMap<String, Nut>,
    path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct RunC {
    data: FlData,
    args: Vec<String>,
    tnut: Nut,
    pnut: Nut,
    tliter: f64,
    objects: Objects,
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
            objects: Objects {
                path: PathBuf::new(),
                objects: HashMap::new(),
            },
        }
    }
    pub fn init(&mut self) {
        self.args = env::args().collect();
        self.args.remove(0);
        self.data.parse_args(&self.args);
        d_log!("-> parsing args, done");
        self.args.clear();
        d_log!("-> clearing args, done");
        if !check_file(&self.data.file) {
            self.data.file = PathBuf::from("/home/wassim/foo/cal/data/data.yaml");
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

        self.objects.path = PathBuf::from("/home/wassim/foo/cal/data/objects.yaml");
        let contents = fs::read_to_string(&self.objects.path).unwrap_or_default();
        self.objects.objects = serde_yaml::from_str(&contents).unwrap_or_default();
    }
    pub fn run(mut self) {
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

            match self.objects.objects.get(s.as_str()) {
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
            match get_w_add(s) {
                Nwc::C(cal) => self.tnut.cal += cal,
                Nwc::W(wtr) => self.tliter += wtr,
                Nwc::N(tname, val) => {
                    let mut obj = self.objects.objects.get(tname).unwrap().clone();
                    obj.scal(val);
                    self.tnut.add(&obj);
                }
            }
        }
        d_log!("heres total nut : {:#?}", self.tnut);
        println!("=> total nuts\n");
        self.tnut.print();
        if !self.data.runmod.minimal {
            println!("\n=> left nuts\n");
            self.tnut.printb(&self.pnut);
        }
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
        for key in self.objects.objects.keys() {
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
