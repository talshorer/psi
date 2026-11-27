use std::{collections::BTreeMap, fmt::Write as _, ops::Deref};

use euclid::{Point2D, Scale, Vector2D};
use psi_topology::layout::{Boundary, LandNum, Layout, Terrain};

mod for_seg_vor;
mod force_only;
mod poly;
mod texture;
mod units;

use poly::{Poly, PolyBuilder};

pub use texture::PsiSvgTextures;
pub use units::UnitSquarePoint;

use crate::units::{Pixel, Pure, UnitSquare};

const PER_BOUNDARY: Scale<f64, Pure, UnitSquare> = const {
    assert!(Boundary::MIN == 1);
    Scale::new(1.0 / (Boundary::MAX + 1) as f64)
};

#[derive(Debug, Clone, Copy)]
struct NormalizedVec2(Vector2D<f64, Pure>);

impl NormalizedVec2 {
    const fn const_new(x: f64, y: f64) -> Option<Self> {
        if (x * x + y * y) == 1.0 {
            Some(Self(Vector2D::new(x, y)))
        } else {
            None
        }
    }
}

impl<U> From<Vector2D<f64, U>> for NormalizedVec2 {
    fn from(v: Vector2D<f64, U>) -> Self {
        Self(v * Scale::new(1.0 / v.length()))
    }
}

impl Deref for NormalizedVec2 {
    type Target = Vector2D<f64, Pure>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn within_unit_square(point: UnitSquarePoint) -> bool {
    point.x >= 0.0 && point.x <= 1.0 && point.y >= 0.0 && point.y <= 1.0
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

    fn transform(&self, v: UnitSquarePoint) -> UnitSquarePoint {
        UnitSquarePoint::new(v.x + self.cos * v.y, self.sin * v.y)
    }
}

fn ocean(tilt: &Tilt) -> Poly {
    let mut poly = PolyBuilder::default();
    poly.push(UnitSquarePoint::new(0.0, 0.0));
    poly.push(UnitSquarePoint::new(0.0, 1.0));
    poly.push(UnitSquarePoint::new(-tilt.cos, 1.0));
    poly.poly()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment(pub UnitSquarePoint, pub UnitSquarePoint);

impl Segment {
    pub fn crosses(&self, other: &Self) -> Option<UnitSquarePoint> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
        let (v1, v2) = (self.0, self.1);
        let (v3, v4) = (other.0, other.1);
        let diff1 = v2 - v1;
        let diff2 = v4 - v3;
        let det = diff1.x * diff2.y - diff1.y * diff2.x;
        if det == 0.0 {
            return None;
        }
        let t = ((v3.x - v1.x) * diff2.y - (v3.y - v1.y) * diff2.x) / det;
        let u = ((v3.x - v1.x) * diff1.y - (v3.y - v1.y) * diff1.x) / det;
        const ZERO: f64 = 0.000001;
        const ONE: f64 = 0.999999;
        (t > ZERO && t < ONE && u > ZERO && u < ONE).then_some(v1 + (v2 - v1) * t)
    }
}

type PixelScale = Scale<f64, UnitSquare, Pixel>;

fn poly_path(poly: &Poly, scale: PixelScale, tilt: &Tilt) -> Result<String, std::fmt::Error> {
    let mut path = String::new();
    for (i, segment) in poly.iter().enumerate() {
        let directive = if i == 0 { 'M' } else { 'L' };
        let Segment(v0, v1) = segment;
        let Point2D { x: x0, y: y0, .. } = tilt.transform(*v0) * scale;
        let Point2D { x: x1, y: y1, .. } = tilt.transform(*v1) * scale;
        let height = tilt.sin * scale.get();
        let y0 = height - y0;
        let y1 = height - y1;
        write!(&mut path, "{directive} {x0} {y0} {x1} {y1} ")?
    }
    write!(&mut path, "Z")?;
    Ok(path)
}

struct PolyLands(BTreeMap<LandNum, (Terrain, Poly)>);

fn emit(
    lands: PolyLands,
    scale: PixelScale,
    tilt: &Tilt,
    textures: &PsiSvgTextures<'_>,
) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    let viewbox = tilt.transform(UnitSquarePoint::new(1.0, 1.0)) * scale;
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
        r#"<defs>\n  <clipPath id="board"><rect x="0" y="0" width="{}" height="{}" /></clipPath>\n"#,
        viewbox.x, viewbox.y,
    )?;
    textures.write_defs(&mut out, scale)?;
    writeln!(&mut out, "</defs>")?;

    writeln!(&mut out, r#"<g clip-path="url(#board)">"#)?;

    for (terrain, poly) in lands
        .0
        .values()
        .chain(std::iter::once(&(Terrain::Ocean, ocean(tilt))))
    {
        let fill = texture::fill(*terrain);

        let path = poly_path(poly, scale, tilt)?;
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

pub enum PsiSvgAlgo {
    ForSegVor,
    ForceOnly(force_only::ForceOnlyConfig),
}

impl Default for PsiSvgAlgo {
    fn default() -> Self {
        Self::ForceOnly(Default::default())
    }
}

pub fn from_layout(
    layout: &Layout,
    scale: f64,
    theta: f64,
    algo: &PsiSvgAlgo,
    textures: &PsiSvgTextures<'_>,
) -> Result<String, PsiSvgError> {
    let scale = PixelScale::new(scale);
    let tilt = Tilt::new(theta);
    let poly_lands = match algo {
        PsiSvgAlgo::ForSegVor => for_seg_vor::for_seg_vor(layout)?,
        PsiSvgAlgo::ForceOnly(config) => force_only::force_only(layout, config)?,
    };
    let svg = emit(poly_lands, scale, &tilt, textures)?;
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn force_only() -> PsiSvgAlgo {
        let default = force_only::ForceOnlyConfig::default();
        PsiSvgAlgo::ForceOnly(force_only::ForceOnlyConfig {
            spring_force: default.spring_force * Scale::new(10.0),
            pressure: default.pressure * Scale::new(10.0),
            spring_force2: default.spring_force2 * Scale::new(10.0),
            edge_push_force: default.edge_push_force * Scale::new(10.0),
            spring_iters: default.spring_iters / 10,
            deflate_iters: default.deflate_iters / 10,
        })
    }

    #[rstest::rstest]
    fn consistent_output(#[values(PsiSvgAlgo::ForSegVor, force_only())] algo: PsiSvgAlgo) {
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
            let factory =
                || from_layout(&layout, SCALE, THETA, &algo, &PsiSvgTextures::default()).unwrap();
            let svg = factory();
            assert!(outputs.insert(svg.clone()));
            for _ in 0..2 {
                let svg2 = factory();
                assert_eq!(svg, svg2);
            }
        }
    }
}
