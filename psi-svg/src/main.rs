use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};
use psi_svg::{PsiSvgAlgo, PsiSvgTextures};

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

#[derive(Default, Clone, ValueEnum)]
#[value(rename_all = "snake")]
enum Algo {
    ForSegVor,
    #[default]
    ForceOnly,
}

impl Algo {
    fn algo(self, config: Option<&Path>) -> Result<PsiSvgAlgo, Box<dyn Error>> {
        Ok(match self {
            Self::ForSegVor => PsiSvgAlgo::ForSegVor,
            Self::ForceOnly => PsiSvgAlgo::ForceOnly(
                config
                    .map(fs::read_to_string)
                    .transpose()?
                    .map(|data| json5::from_str(&data))
                    .transpose()?
                    .unwrap_or_default(),
            ),
        })
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    /// Algorithm to use
    algo: Option<Algo>,
    /// Algo config
    #[arg(long, short)]
    config: Option<PathBuf>,
    /// Board tilt angle (degrees)
    #[arg(long, short)]
    theta: Option<u32>,
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
    let svg = psi_svg::from_layout(
        &layout,
        700.0,
        args.theta.unwrap_or(60) as f64 * std::f64::consts::PI / 180.0,
        &args.algo.unwrap_or_default().algo(args.config.as_deref())?,
        &PsiSvgTextures::default(),
    )?;
    print!("{}", svg);
    Ok(())
}
