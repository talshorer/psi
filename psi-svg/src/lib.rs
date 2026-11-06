use std::{collections::BTreeMap, fmt::Write as _};

use psi_topology::layout::{Boundary, LandNum, Layout, Terrain};

mod for_seg_vor;
mod poly;
mod vec;

use poly::Poly;
use vec::Vec2;

struct BoundingBox {
    side: f64,
    per_boundary: f64,
}

const _: () = const { assert!(Boundary::MIN == 1) };

impl BoundingBox {
    fn new(scale: f64) -> Self {
        Self {
            side: scale,
            per_boundary: scale / (Boundary::MAX + 1) as f64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Segment(Vec2, Vec2);

fn poly_path(poly: &Poly, bb: &BoundingBox) -> Result<String, std::fmt::Error> {
    let mut path = String::new();
    for (i, segment) in poly.iter().enumerate() {
        let directive = if i == 0 { 'M' } else { 'L' };
        let Segment(Vec2 { x: x0, y: y0 }, Vec2 { x: x1, y: y1 }) = segment;
        let y0 = bb.side - y0;
        let y1 = bb.side - y1;
        write!(&mut path, "{directive} {x0} {y0} {x1} {y1} ")?
    }
    write!(&mut path, "Z")?;
    Ok(path)
}

struct PolyLands(BTreeMap<LandNum, (Terrain, Poly)>);

fn emit(lands: PolyLands, bb: &BoundingBox) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    writeln!(
        &mut out,
        r#"<svg xmlns="http://www.w3.org/2000/svg"
         viewBox="0 0 {0} {0}"
         width="{0}"
         height="{0}">"#,
        bb.side,
    )?;

    writeln!(
        &mut out,
        r#"<defs>
         <clipPath id="board">
             <rect x="0" y="0" width="{0}" height="{0}" />
         </clipPath>
       </defs>"#,
        bb.side,
    )?;

    writeln!(&mut out, r#"<g clip-path="url(#board)">"#)?;

    for (terrain, poly) in lands.0.values() {
        let fill = match terrain {
            Terrain::Jungle => "green",
            Terrain::Mountain => "grey",
            Terrain::Sands => "gold",
            Terrain::Wetlands => "teal",
            Terrain::Ocean => "blue",
        };

        let path = poly_path(poly, bb)?;
        writeln!(
            &mut out,
            r#"<path d="{path}" fill="{fill}" stroke="black" stroke-width="1" />"#
        )?;
    }

    writeln!(&mut out, "</g>")?;
    writeln!(&mut out, "</svg>")?;

    Ok(out)
}

#[derive(Debug, thiserror::Error)]
pub enum PsiSvgError {
    #[error("ForSegVor({0})")]
    ForSegVor(#[from] for_seg_vor::ForSegVorError),
    #[error("Fmt({0})")]
    Fmt(#[from] std::fmt::Error),
}

pub fn from_layout(layout: &Layout, scale: f64) -> Result<String, PsiSvgError> {
    let bb = BoundingBox::new(scale);
    let poly_lands = for_seg_vor::for_seg_vor(layout, &bb)?;
    let svg = emit(poly_lands, &bb)?;
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn consistent_output() {
        const SCALE: f64 = 1000.0;
        let mut outputs = HashSet::new();
        for layout in [
            psi_topology::layout::data::A.layout(),
            psi_topology::layout::data::B.layout(),
            psi_topology::layout::data::C.layout(),
            psi_topology::layout::data::D.layout(),
            psi_topology::layout::data::E.layout(),
            psi_topology::layout::data::F.layout(),
            psi_topology::layout::data::G.layout(),
            psi_topology::layout::data::H.layout(),
        ] {
            let factory = || from_layout(&layout, SCALE).unwrap();
            let svg = factory();
            assert!(outputs.insert(svg.clone()));
            for _ in 0..2 {
                let svg2 = factory();
                assert_eq!(svg, svg2);
            }
        }
    }
}
