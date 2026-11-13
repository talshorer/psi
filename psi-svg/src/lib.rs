use std::{collections::BTreeMap, fmt::Write as _};

use psi_topology::layout::{Boundary, LandNum, Layout, Terrain};
use psi_vec::Vec2;

mod for_seg_vor;
mod force_only;
mod poly;

use poly::{Poly, PolyBuilder};

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

struct Tilt {
    sin: f64,
    cos: f64,
}

impl Tilt {
    fn new(theta: f64) -> Self {
        Self {
            sin: f64::sin(theta),
            cos: f64::cos(theta),
        }
    }

    fn transform(&self, v: Vec2) -> Vec2 {
        Vec2 {
            x: v.x + self.cos * v.y,
            y: self.sin * v.y,
        }
    }
}

fn ocean(bb: &BoundingBox, tilt: &Tilt) -> Poly {
    let mut poly = PolyBuilder::default();
    let points = [
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: 0.0, y: bb.side },
        Vec2 {
            x: -tilt.cos * bb.side,
            y: bb.side,
        },
    ];
    for (v1, v2) in points.iter().zip(points.iter().cycle().skip(1)) {
        poly.push(Segment(*v1, *v2));
    }
    poly.poly()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Segment(Vec2, Vec2);

fn poly_path(poly: &Poly, bb: &BoundingBox, tilt: &Tilt) -> Result<String, std::fmt::Error> {
    let mut path = String::new();
    for (i, segment) in poly.iter().enumerate() {
        let directive = if i == 0 { 'M' } else { 'L' };
        let Segment(v0, v1) = segment;
        let Vec2 { x: x0, y: y0 } = tilt.transform(*v0);
        let Vec2 { x: x1, y: y1 } = tilt.transform(*v1);
        let height = tilt.sin * bb.side;
        let y0 = height - y0;
        let y1 = height - y1;
        write!(&mut path, "{directive} {x0} {y0} {x1} {y1} ")?
    }
    write!(&mut path, "Z")?;
    Ok(path)
}

struct PolyLands(BTreeMap<LandNum, (Terrain, Poly)>);

fn emit(lands: PolyLands, bb: &BoundingBox, tilt: &Tilt) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    let viewbox = tilt.transform(Vec2 {
        x: bb.side,
        y: bb.side,
    });
    writeln!(
        &mut out,
        r#"<svg xmlns="http://www.w3.org/2000/svg"
         viewBox="0 0 {0} {1}"
         width="{0}"
         height="{1}">"#,
        viewbox.x, viewbox.y,
    )?;

    writeln!(
        &mut out,
        r#"<defs>
         <clipPath id="board">
             <rect x="0" y="0" width="{}" height="{}" />
         </clipPath>
       </defs>"#,
        viewbox.x, viewbox.y,
    )?;

    writeln!(&mut out, r#"<g clip-path="url(#board)">"#)?;

    for (terrain, poly) in lands
        .0
        .values()
        .chain(std::iter::once(&(Terrain::Ocean, ocean(bb, tilt))))
    {
        let fill = match terrain {
            Terrain::Jungle => "green",
            Terrain::Mountain => "grey",
            Terrain::Sands => "gold",
            Terrain::Wetlands => "teal",
            Terrain::Ocean => "blue",
        };

        let path = poly_path(poly, bb, tilt)?;
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
    #[error("ForceOnly({0})")]
    ForceOnly(#[from] force_only::ForceOnlyError),
    #[error("Fmt({0})")]
    Fmt(#[from] std::fmt::Error),
}

#[derive(Default, Clone, Copy)]
pub enum PsiSvgAlgo {
    ForSegVor,
    #[default]
    ForceOnly,
}

pub fn from_layout(
    layout: &Layout,
    scale: f64,
    theta: f64,
    algo: PsiSvgAlgo,
) -> Result<String, PsiSvgError> {
    let bb = BoundingBox::new(scale);
    let tilt = Tilt::new(theta);
    let poly_lands = match algo {
        PsiSvgAlgo::ForSegVor => for_seg_vor::for_seg_vor(layout, &bb)?,
        PsiSvgAlgo::ForceOnly => force_only::force_only(layout, &bb)?,
    };
    let svg = emit(poly_lands, &bb, &tilt)?;
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[rstest::rstest]
    fn consistent_output(#[values(PsiSvgAlgo::ForSegVor, PsiSvgAlgo::ForceOnly)] algo: PsiSvgAlgo) {
        const SCALE: f64 = 1000.0;
        const THETA: f64 = std::f64::consts::FRAC_PI_2;
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
            let factory = || from_layout(&layout, SCALE, THETA, algo).unwrap();
            let svg = factory();
            assert!(outputs.insert(svg.clone()));
            for _ in 0..2 {
                let svg2 = factory();
                assert_eq!(svg, svg2);
            }
        }
    }
}
