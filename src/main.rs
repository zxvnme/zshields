#![feature(proc_macro_hygiene, decl_macro, rustc_private)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

use rocket::response::content::Html;

mod projects;
use projects::zgui::funcs::*;

#[catch(404)]
fn not_found(req: &rocket::Request) -> Html<String> {
    Html(format!("<h3>Sorry but '{}' doesnt seem to be valid.</h3>
                  <p>try one of them instead:<p>
                  <ul>
                  <li>/zgui/version</li>
                  <li>/zgui/version_type</li>
                  <li>/zgui/discord</li>
                  </ul>
                  <h3>zshields made with ❤️</h3>", req.uri()))
}

fn main() {
    rocket::ignite()
        .mount("/zgui", routes![zgui_version, zgui_version_type, zgui_discord])
        .register(catchers![not_found])
        .launch();
}