use serde::{Deserialize, Serialize};

use crate::{
    d_log,
    printing::{Color, left_right, progress_bar},
};
use std::{fs, io::Write, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nut {
    pub cal: f64,
    pub carb: f64,
    pub prot: f64,
    pub fiber: f64,
    pub fat: f64,
}

impl<'a> IntoIterator for &'a mut Nut {
    type Item = &'a mut f64;
    type IntoIter = std::array::IntoIter<&'a mut f64, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &mut self.carb,
            &mut self.cal,
            &mut self.prot,
            &mut self.fiber,
            &mut self.fat,
        ]
        .into_iter()
    }
}

impl Nut {
    pub fn add(&mut self, other: &Nut) {
        self.cal += other.cal;
        self.carb += other.carb;
        self.prot += other.prot;
        self.fiber += other.fiber;
        self.fat += other.fat;
    }
    pub fn scal(&mut self, sc: f64) {
        self.carb *= sc;
        self.prot *= sc;
        self.cal *= sc;
        self.fiber *= sc;
        self.fat *= sc;
    }
    pub fn print(&self) {
        print!(
            "- calories : {:.2}\n- carbs : {:.2}\n- proteins: {:.2}\n- fiber: {:.2}\n- fat: {:.2}\n",
            self.cal, self.carb, self.prot, self.fiber, self.fat
        );
    }
    pub fn printb(&self, needs: &Nut) {
        print!(
            "{}\n{}\n{}\n{}\n",
            left_right(
                &format!("carbs : {:.2}", self.carb),
                &progress_bar(needs.carb - self.carb, needs.carb, 25, Color::Orange),
                53
            ),
            left_right(
                &format!("proteins : {:.2}", self.prot),
                &progress_bar(needs.prot - self.prot, needs.prot, 25, Color::Grey),
                53
            ),
            left_right(
                &format!("fat : {:.2}", self.fat),
                &progress_bar(needs.fat - self.fat, needs.fat, 25, Color::Amber),
                53
            ),
            left_right(
                &format!("fiber : {:.2}", self.fiber),
                &progress_bar(needs.fiber - self.fiber, needs.fiber, 25, Color::Green),
                53
            ),
        );
    }
    pub fn new() -> Self {
        Self {
            cal: 0.0,
            carb: 0.0,
            prot: 0.0,
            fiber: 0.0,
            fat: 0.0,
        }
    }
}

pub fn get_nut_from_file<P: AsRef<Path>>(path: P) -> Nut {
    d_log!("==> getting nut from file {}", path.as_ref().display());
    let contents = fs::read_to_string(path).unwrap_or_default();
    match serde_yaml::from_str(&contents) {
        Ok(nu) => nu,
        Err(err) => {
            eprint!("error : {err}");
            Nut {
                cal: 0.0,
                carb: 0.0,
                prot: 0.0,
                fiber: 0.0,
                fat: 0.0,
            }
        }
    }
}
pub fn store_nut_to_file<P: AsRef<Path>>(
    path: P,
    nuts: &Nut,
) -> Result<(), Box<dyn std::error::Error>> {
    d_log!(
        "==> storing nut to file with path {} and nuts {nuts:?}",
        path.as_ref().display()
    );
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    let contents = serde_yaml::to_string(&nuts)?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(contents.as_bytes())?;
    Ok(())
}
