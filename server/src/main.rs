#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

extern crate event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ew");
    event::init();

    Ok(())
}
