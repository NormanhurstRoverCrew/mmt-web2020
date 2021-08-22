#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::config::Environment;
use rocket::response::{content::Html, NamedFile};
use rocket::Config;

#[get("/<file..>")]
fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&file))
        .or_else(|_| NamedFile::open(Path::new("index.html")))
        .ok()
}

#[get("/")]
fn root() -> Option<Html<NamedFile>> {
    NamedFile::open(Path::new("index.html"))
        .ok()
        .map(|f| Html(f))
}

fn rocket() -> rocket::Rocket {
    rocket::custom(
        Config::build(Environment::Development)
            .address("0.0.0.0")
            .unwrap(),
    )
    .mount("/", routes![all, root])
}

fn main() {
    rocket().launch();
}
