use serde::{Deserialize, Serialize};

use crate::{
    d_log,
    printing::{Color, left_right, progress_bar},
};
use std::{collections::HashMap, fs, path::Path};

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
    let contents = serde_yaml::to_string(&nuts)?;
    fs::write(path, contents)?;
    Ok(())
}
pub fn build_objects() -> HashMap<&'static str, Nut> {
    let mut map = HashMap::new();
    map.insert(
        "apple",
        Nut {
            cal: 0.52,
            carb: 0.14,
            prot: 0.0,
            fiber: 0.02,
            fat: 0.0,
        },
    );
    map.insert(
        "avocado",
        Nut {
            cal: 1.6,
            carb: 0.085,
            prot: 0.02,
            fiber: 0.067,
            fat: 0.147,
        },
    );
    map.insert(
        "almond",
        Nut {
            cal: 5.8,
            carb: 0.21,
            prot: 0.0,
            fiber: 0.12,
            fat: 0.5,
        },
    );
    map.insert(
        "amlo",
        Nut {
            cal: 5.58,
            carb: 0.338,
            prot: 0.122,
            fiber: 0.069,
            fat: 0.453,
        },
    );
    map.insert(
        "bread",
        Nut {
            cal: 2.7,
            carb: 0.50,
            prot: 0.12,
            fiber: 0.02,
            fat: 0.02,
        },
    );
    map.insert(
        "bisara",
        Nut {
            cal: 0.85,
            carb: 0.13,
            prot: 0.15,
            fiber: 0.02,
            fat: 0.0,
        },
    );
    map.insert(
        "banana",
        Nut {
            cal: 0.89,
            carb: 0.13,
            prot: 0.15,
            fiber: 0.02,
            fat: 0.0,
        },
    );
    map.insert(
        "beans",
        Nut {
            cal: 1.3,
            carb: 0.25,
            prot: 0.06,
            fiber: 0.15,
            fat: 0.0,
        },
    );
    map.insert(
        "beef",
        Nut {
            cal: 2.5,
            carb: 0.0,
            prot: 0.26,
            fiber: 0.0,
            fat: 0.15,
        },
    );
    map.insert(
        "chickpeas",
        Nut {
            cal: 1.64,
            carb: 0.27,
            prot: 0.089,
            fiber: 0.076,
            fat: 0.02,
        },
    );

    map.insert(
        "carrot",
        Nut {
            cal: 0.41,
            carb: 0.1,
            prot: 0.03,
            fiber: 0.028,
            fat: 0.0,
        },
    );
    map.insert(
        "chickenb",
        Nut {
            cal: 1.2,
            carb: 0.0,
            prot: 0.22,
            fiber: 0.0,
            fat: 0.03,
        },
    );
    map.insert(
        "chocolate",
        Nut {
            cal: 5.3,
            carb: 0.0,
            prot: 0.0,
            fiber: 0.03,
            fat: 0.3,
        },
    );
    map.insert(
        "dates",
        Nut {
            cal: 2.77,
            carb: 0.75,
            prot: 0.12,
            fiber: 0.07,
            fat: 0.0,
        },
    );
    map.insert(
        "eggs",
        Nut {
            cal: 1.55,
            carb: 0.01,
            prot: 0.13,
            fiber: 0.0,
            fat: 0.11,
        },
    );
    map.insert(
        "fish",
        Nut {
            cal: 1.5,
            carb: 0.0,
            prot: 0.28,
            fiber: 0.0,
            fat: 0.07,
        },
    );
    map.insert(
        "fava",
        Nut {
            cal: 0.3,
            carb: 0.065,
            prot: 0.025,
            fiber: 0.2,
            fat: 0.002,
        },
    );
    map.insert(
        "grape",
        Nut {
            cal: 0.69,
            carb: 0.18,
            prot: 0.0,
            fiber: 0.009,
            fat: 0.0,
        },
    );
    map.insert(
        "honey",
        Nut {
            cal: 3.04,
            carb: 0.82,
            prot: 0.12,
            fiber: 0.002,
            fat: 0.0,
        },
    );
    map.insert(
        "lentils",
        Nut {
            cal: 1.16,
            carb: 0.2,
            prot: 0.09,
            fiber: 0.079,
            fat: 0.0,
        },
    );
    map.insert(
        "lamb",
        Nut {
            cal: 2.94,
            carb: 0.0,
            prot: 0.25,
            fiber: 0.0,
            fat: 0.1,
        },
    );
    map.insert(
        "mango",
        Nut {
            cal: 0.6,
            carb: 0.15,
            prot: 0.0,
            fiber: 0.016,
            fat: 0.0,
        },
    );
    map.insert(
        "melon",
        Nut {
            cal: 0.3,
            carb: 0.08,
            prot: 0.0,
            fiber: 0.008,
            fat: 0.0,
        },
    );
    map.insert(
        "orange",
        Nut {
            cal: 0.47,
            carb: 0.12,
            prot: 0.01,
            fiber: 0.025,
            fat: 0.0,
        },
    );
    map.insert(
        "onion",
        Nut {
            cal: 0.4,
            carb: 0.09,
            prot: 0.01,
            fiber: 0.02,
            fat: 0.0,
        },
    );
    map.insert(
        "oil",
        Nut {
            cal: 9.0,
            carb: 0.0,
            prot: 0.0,
            fiber: 0.0,
            fat: 1.0,
        },
    );
    map.insert(
        "oats",
        Nut {
            cal: 4.3,
            carb: 0.66,
            prot: 0.12,
            fiber: 0.1,
            fat: 0.06,
        },
    );
    map.insert(
        "pear",
        Nut {
            cal: 0.57,
            carb: 0.15,
            prot: 0.0,
            fiber: 0.03,
            fat: 0.0,
        },
    );
    map.insert(
        "potato",
        Nut {
            cal: 0.75,
            carb: 0.2,
            prot: 0.02,
            fiber: 0.02,
            fat: 0.0,
        },
    );
    map.insert(
        "peanut",
        Nut {
            cal: 6.67,
            carb: 0.16,
            prot: 0.25,
            fiber: 0.08,
            fat: 0.49,
        },
    );
    map.insert(
        "pntbutter",
        Nut {
            cal: 6.25,
            carb: 0.0,
            prot: 0.0,
            fiber: 0.05,
            fat: 0.5,
        },
    );
    map.insert(
        "rice",
        Nut {
            cal: 1.3,
            carb: 0.28,
            prot: 0.06,
            fiber: 0.004,
            fat: 0.003,
        },
    );
    map.insert(
        "raisins",
        Nut {
            cal: 3.0,
            carb: 0.79,
            prot: 0.0,
            fiber: 0.037,
            fat: 0.0,
        },
    );
    map.insert(
        "squid",
        Nut {
            cal: 0.92,
            carb: 0.02,
            prot: 0.15,
            fiber: 0.0,
            fat: 0.013,
        },
    );
    map.insert(
        "tomato",
        Nut {
            cal: 0.18,
            carb: 0.04,
            prot: 0.0,
            fiber: 0.01,
            fat: 0.0,
        },
    );
    map
}
