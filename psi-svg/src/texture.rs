use std::fmt::Write as _;
use std::io::Cursor;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use psi_topology::layout::Terrain;

pub(crate) fn fill(terrain: Terrain) -> &'static str {
    match terrain {
        Terrain::Jungle => "url(#texture-jungle)",
        Terrain::Mountain => "url(#texture-mountain)",
        Terrain::Sands => "url(#texture-sands)",
        Terrain::Wetlands => "url(#texture-wetlands)",
        Terrain::Ocean => "url(#texture-ocean)",
    }
}

pub(crate) fn write_defs(out: &mut String, scale: f64) -> std::fmt::Result {
    write_texture_pattern(
        out,
        "jungle",
        include_bytes!("../assets/terrain-textures/jungle.png"),
        scale,
    )?;
    write_texture_pattern(
        out,
        "mountain",
        include_bytes!("../assets/terrain-textures/mountain.png"),
        scale,
    )?;
    write_texture_pattern(
        out,
        "sands",
        include_bytes!("../assets/terrain-textures/sands.png"),
        scale,
    )?;
    write_texture_pattern(
        out,
        "wetlands",
        include_bytes!("../assets/terrain-textures/wetlands.png"),
        scale,
    )?;
    write_texture_pattern(
        out,
        "ocean",
        include_bytes!("../assets/terrain-textures/ocean.png"),
        scale,
    )?;
    Ok(())
}

fn write_texture_pattern(out: &mut String, id: &str, png: &[u8], scale: f64) -> std::fmt::Result {
    let (pixel_width, pixel_height) = png_dimensions(png);
    let width = pixel_width as f64 * scale;
    let height = pixel_height as f64 * scale;
    let data_uri = format!("data:image/png;base64,{}", STANDARD.encode(png));
    writeln!(
        out,
        r#"  <pattern id="texture-{id}" patternUnits="userSpaceOnUse" x="0" y="0" width="{width}" height="{height}">\n"#
    )?;
    writeln!(
        out,
        r#"    <image x="0" y="0" width="{width}" height="{height}" href="{data_uri}" />\n"#
    )?;
    writeln!(out, "  </pattern>")
}

fn png_dimensions(png: &[u8]) -> (u32, u32) {
    let decoder = png::Decoder::new(Cursor::new(png));
    let reader = decoder
        .read_info()
        .expect("failed to read terrain texture PNG header");
    let info = reader.info();
    (info.width, info.height)
}
