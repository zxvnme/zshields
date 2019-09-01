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
        "labelColor": "000000",
        "style": "for-the-badge",
        "color": "212121",
        "message": json.version,
    }))
}

#[get("/version_type")]
fn zgui_version_type() -> Result<JsonValue, reqwest::Error> {
    let response = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/zxvnme/zgui/zgui-2.0/zgui.json")
        .send()?
        .text()?;

    let json: ZGUIVersionShield = serde_json::from_str(response.as_str()).unwrap();

    Ok(json!({
        "schemaVersion": 1,
        "label": "TYPE",
        "labelColor": "000000",
        "style": "for-the-badge",
        "color": "212121",
        "message": json.version_type,
    }))
}

#[get("/discord")]
fn zgui_discord() -> Result<JsonValue, reqwest::Error> {
    Ok(json!({
        "schemaVersion": 1,
        "label": "JOIN",
        "labelColor": "7289DA",
        "namedLogo": "Discord",
        "logoColor": "FFFFFF",
        "style": "for-the-badge",
        "color": "212121",
        "message": "THE COMMUNITY",
    }))
}

fn main() {
    rocket::ignite()
        .mount("/zgui", routes![zgui_version, zgui_version_type, zgui_discord])
        .launch();
}