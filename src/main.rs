#![feature(proc_macro_hygiene, decl_macro, rustc_private)]
extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use crate::structs::ZGUIVersionShield;
mod structs;

#[get("/version")]
fn zgui_version() -> Result<JsonValue, reqwest::Error> {
    let response = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/zxvnme/zgui/zgui-2.0/zgui.json")
        .send()?
        .text()?;

    let json: ZGUIVersionShield = serde_json::from_str(response.as_str()).unwrap();

    Ok(json!({
        "schemaVersion": 1,
        "label": "VERSION",
        "style": "for-the-badge",
        "message": json.version,
        "color": "000000"
    }))
}

fn main() {
    rocket::ignite()
        .mount("/zgui", routes![zgui_version])
        .launch();
}