#![feature(proc_macro_hygiene, decl_macro, rustc_private)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod projects;
use projects::zgui::funcs::*;

fn main() {
    rocket::ignite()
        .mount("/zgui",
               routes![zgui_version,
                              zgui_version_type,
                              zgui_discord])
        .launch();
}