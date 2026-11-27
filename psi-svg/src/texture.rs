use std::io::Cursor;
use std::{borrow::Cow, fmt::Write as _};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use psi_topology::layout::Terrain;

#[derive(Default)]
pub struct PsiSvgTextures<'a> {
    pub jungle: Option<Cow<'a, [u8]>>,
    pub mountain: Option<Cow<'a, [u8]>>,
    pub sands: Option<Cow<'a, [u8]>>,
    pub wetlands: Option<Cow<'a, [u8]>>,
    pub ocean: Option<Cow<'a, [u8]>>,
}

impl<'a> PsiSvgTextures<'a> {
    const BASE_SCALE: f64 = 700.0;

    pub(crate) fn write_defs(&self, out: &mut String, scale: f64) -> std::fmt::Result {
        let scale = scale / Self::BASE_SCALE;
        write_texture_pattern(
            out,
            "jungle",
            self.jungle
                .as_ref()
                .map(Cow::as_ref)
                .unwrap_or(include_bytes!("../assets/terrain-textures/jungle.png")),
            scale,
        )?;
        write_texture_pattern(
            out,
            "mountain",
            self.mountain
                .as_ref()
                .map(Cow::as_ref)
                .unwrap_or(include_bytes!("../assets/terrain-textures/mountain.png")),
            scale,
        )?;
        write_texture_pattern(
            out,
            "sands",
            self.sands
                .as_ref()
                .map(Cow::as_ref)
                .unwrap_or(include_bytes!("../assets/terrain-textures/sands.png")),
            scale,
        )?;
        write_texture_pattern(
            out,
            "wetlands",
            self.wetlands
                .as_ref()
                .map(Cow::as_ref)
                .unwrap_or(include_bytes!("../assets/terrain-textures/wetlands.png")),
            scale,
        )?;
        write_texture_pattern(
            out,
            "ocean",
            self.ocean
                .as_ref()
                .map(Cow::as_ref)
                .unwrap_or(include_bytes!("../assets/terrain-textures/ocean.png")),
            scale,
        )?;
        Ok(())
    }
}

pub(crate) fn fill(terrain: Terrain) -> &'static str {
    match terrain {
        Terrain::Jungle => "url(#texture-jungle)",
        Terrain::Mountain => "url(#texture-mountain)",
        Terrain::Sands => "url(#texture-sands)",
        Terrain::Wetlands => "url(#texture-wetlands)",
        Terrain::Ocean => "url(#texture-ocean)",
    }
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
