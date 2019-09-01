use rocket_contrib::json::JsonValue;

use crate::projects::zgui::structs;
use crate::projects::zgui::utils;

#[get("/version")]
pub fn zgui_version() -> Result<JsonValue, reqwest::Error> {
    let zgui: structs::ZGUIRepositoryInfo = serde_json::from_str(utils::get_zgui_info()?.as_str()).unwrap();

    Ok(json!({
        "schemaVersion": 1,
        "label": "VERSION",
        "labelColor": "000000",
        "style": "for-the-badge",
        "color": "212121",
        "message": zgui.version,
    }))
}

#[get("/version_type")]
pub fn zgui_version_type() -> Result<JsonValue, reqwest::Error> {
    let zgui: structs::ZGUIRepositoryInfo = serde_json::from_str(utils::get_zgui_info()?.as_str()).unwrap();

    Ok(json!({
        "schemaVersion": 1,
        "label": "TYPE",
        "labelColor": "000000",
        "style": "for-the-badge",
        "color": "212121",
        "message": zgui.version_type,
    }))
}

#[get("/discord")]
pub fn zgui_discord() -> Result<JsonValue, reqwest::Error> {
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
