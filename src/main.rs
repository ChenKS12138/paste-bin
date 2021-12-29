#[macro_use]
extern crate lazy_static;

mod app;
mod cmd;

fn main() {
    cmd::boost().unwrap();
}
