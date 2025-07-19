use anyhow::{Result, bail};
use hex::FromHex;
use serde_json::Value;

const GENERAL_SECTION: &str = "[General]
Description=Stylix
Wallpaper=
";

fn json_value_to_rgb(palette: &Value, color_name: &str) -> Result<String> {
    let Some(color) = palette.get(color_name) else {
        bail!("palette does not define color {color_name:?}");
    };
    let Some(color_hex) = color.as_str() else {
        bail!("the color {color_name:?} is not defined as a string");
    };
    let [r, g, b] = <[u8; 3]>::from_hex(color_hex)?;
    Ok(format!("{r},{g},{b}"))
}

fn main() -> Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        bail!("please supply a path to a mustache JSON file containing the colorscheme");
    };
    let palette: Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;

    let colors: Vec<_> = (0..16)
        .map(|base| json_value_to_rgb(&palette, &format!("base{base:02X}")))
        .collect::<Result<_>>()?;

    let sections: Vec<_> = [
        ("Background", 0),
        ("BackgroundIntense", 3),
        ("Color0", 0),
        ("Color0Intense", 3),
        ("Color1", 8),
        ("Color1Intense", 8),
        ("Color2", 0xB),
        ("Color2Intense", 0xB),
        ("Color3", 0xA),
        ("Color3Intense", 0xA),
        ("Color4", 0xD),
        ("Color4Intense", 0xD),
        ("Color5", 0xE),
        ("Color5Intense", 0xE),
        ("Color6", 0xC),
        ("Color6Intense", 0xC),
        ("Color7", 5),
        ("Color7Intense", 7),
        ("Foreground", 5),
        ("ForegroundIntense", 7),
    ]
    .into_iter()
    .map(|(name, ix)| format!("[{}]\nColor={}", name, &colors[ix]))
    .collect();
    println!("{GENERAL_SECTION}{}", sections.join("\n"));
    Ok(())
}
