use psi_vec::Vec2;

use crate::Scatter;

pub fn generate_svg(scatter: &Scatter) -> String {
    const WIDTH: f64 = 500.0;
    const HEIGHT: f64 = 500.0;
    const PADDING: f64 = 20.0;

    let scale = |v: Vec2| -> (f64, f64) {
        let x = v.x * WIDTH;
        let y = (1.0 - v.y) * HEIGHT;
        (x, y)
    };

    let mut svg = String::new();

    svg.push_str(&format!(
        r##"<svg viewBox="-{p} -{p} {w} {h}" width="100%" height="{h_raw}" xmlns="http://w3.org" style="background: #111827; border-radius: 8px;">
  <defs>
    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
      <feGaussianBlur stdDeviation="4" result="blur" />
      <feComposite in="SourceGraphic" in2="blur" operator="over" />
    </filter>
  </defs>
"##,
        p = PADDING,
        w = WIDTH + (PADDING * 2.0),
        h = HEIGHT + (PADDING * 2.0),
        h_raw = HEIGHT
    ));

    svg.push_str("  <!-- GRAPH CONNECTIONS (LINKS) -->\n");
    svg.push_str(r###"  <g stroke="#9CA3AF" stroke-width="1.5" stroke-opacity="0.6" stroke-linecap="round">"###);
    svg.push('\n');
    for (start, end) in &scatter.links {
        let v1 = scatter.lands[*start];
        let v2 = scatter.lands[*end];
        let (x1, y1) = scale(v1);
        let (x2, y2) = scale(v2);
        svg.push_str(&format!(
            "    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" />\n",
            x1, y1, x2, y2
        ));
    }
    svg.push_str("  </g>\n");

    svg.push_str("  <!-- NODES -->\n");
    svg.push_str("  <g fill=\"#10B981\">\n");
    for (idx, land) in scatter.lands.iter().enumerate() {
        let (cx, cy) = scale(*land);
        svg.push_str(&format!(
            "    <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"5\" filter=\"url(#glow)\" />\n",
            cx, cy,
        ));
        svg.push_str(&format!(
            "    <text x=\"{:.2}\" y=\"{:.2}\" fill=\"#9CA3AF\"\n",
            cx + 8.0,
            cy + 4.0,
        ));
        svg.push_str(&format!(
            "      font-family=\"sans-serif\" font-size=\"12\" font-weight=\"bold\">{}</text>\n",
            idx + 1,
        ));
    }
    svg.push_str("  </g>\n");

    svg.push_str("</svg>\n");

    svg
}
