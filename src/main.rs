mod debug;
mod flags;
mod objects;
mod parse;
mod printing;
mod run;

fn main() {
    let data = run::RunC::new();
    data.run();
}
