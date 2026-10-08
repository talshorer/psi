use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
#[value(rename_all = "UPPER")]
enum Layout {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Parser)]
struct Args {
    /// Board layout
    layout: Layout,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let layout = match args.layout {
        Layout::A => &psi_topology::layout::data::A,
        Layout::B => &psi_topology::layout::data::B,
        Layout::C => &psi_topology::layout::data::C,
        Layout::D => &psi_topology::layout::data::D,
        Layout::E => &psi_topology::layout::data::E,
        Layout::F => &psi_topology::layout::data::F,
        Layout::G => &psi_topology::layout::data::G,
        Layout::H => &psi_topology::layout::data::H,
    }
    .layout();
    let svg = psi_svg::from_layout(&layout, 700.0)?;
    print!("{}", svg);
    Ok(())
}
