mod debug;
mod flags;
mod objects;
mod parse;
mod printing;
mod run;

fn main() {
    let mut data = run::RunC::new();
    data.init();
    data.run();
}
