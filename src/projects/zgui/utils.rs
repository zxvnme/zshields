pub fn get_zgui_info() -> Result<String, reqwest::Error> {
    let response = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/zxvnme/zgui/zgui-2.0/zgui.json")
        .send()?
        .text()?;

    Ok(response)
}