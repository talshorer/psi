use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

#[derive(serde::Deserialize)]
enum LayoutId {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(serde::Deserialize)]
struct LayoutSvgRequest {
    layout: LayoutId,
    scale: f64,
}

fn validate_scale(scale: f64) -> Result<(), JsError> {
    if !scale.is_finite() {
        Err(JsError::new("Scale must be finite"))
    } else if scale <= 0.0 {
        Err(JsError::new("Scale must be positive"))
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn layout_svg(value: JsValue) -> Result<String, JsError> {
    let LayoutSvgRequest { layout, scale } = serde_wasm_bindgen::from_value(value)?;
    validate_scale(scale)?;
    let layout = match layout {
        LayoutId::A => &psi_topology::layout::data::A,
        LayoutId::B => &psi_topology::layout::data::B,
        LayoutId::C => &psi_topology::layout::data::C,
        LayoutId::D => &psi_topology::layout::data::D,
        LayoutId::E => &psi_topology::layout::data::E,
        LayoutId::F => &psi_topology::layout::data::F,
        LayoutId::G => &psi_topology::layout::data::G,
        LayoutId::H => &psi_topology::layout::data::H,
    }
    .layout();
    let svg = psi_svg::from_layout(
        &layout,
        scale,
        std::f64::consts::FRAC_PI_3,
        &psi_svg::PsiSvgAlgo::ForceOnly(Default::default()),
        &Default::default(),
    )?;
    Ok(svg)
}
