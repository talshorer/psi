use std::fs;

use clap::Parser;
use psi_layout_gen::{
    LayoutGenerator, LayoutGeneratorBalance, LayoutGeneratorBalancedWeights, LayoutGeneratorConfig,
    LayoutGeneratorUnbalancedWeights, WeightedOption,
};
use rand::Rng as _;

#[derive(Parser)]
struct Args {
    #[arg(long, short, default_value_t = false)]
    /// Produce a balanced board
    unbalanced: bool,
    #[arg(long, short)]
    /// Seed to use
    seed: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let layout = LayoutGenerator::new(
        args.seed.unwrap_or_else(|| rand::rng().next_u64()),
        LayoutGeneratorConfig {
            balance: if args.unbalanced {
                LayoutGeneratorBalance::Unbalanced(LayoutGeneratorUnbalancedWeights {
                    coastal_count: vec![
                        WeightedOption {
                            value: 2,
                            weight: 1,
                        },
                        WeightedOption {
                            value: 3,
                            weight: 10,
                        },
                        WeightedOption {
                            value: 4,
                            weight: 3,
                        },
                    ],
                    edge_land_count: vec![
                        WeightedOption {
                            value: 2,
                            weight: 15,
                        },
                        WeightedOption {
                            value: 3,
                            weight: 6,
                        },
                        WeightedOption {
                            value: 4,
                            weight: 1,
                        },
                    ],
                    land_count: vec![
                        WeightedOption {
                            value: 7,
                            weight: 5,
                        },
                        WeightedOption {
                            value: 8,
                            weight: 40,
                        },
                        WeightedOption {
                            value: 9,
                            weight: 5,
                        },
                        WeightedOption {
                            value: 10,
                            weight: 5,
                        },
                        WeightedOption {
                            value: 11,
                            weight: 5,
                        },
                    ],
                    terrain: vec![
                        WeightedOption {
                            value: 'J',
                            weight: 1,
                        },
                        WeightedOption {
                            value: 'M',
                            weight: 1,
                        },
                        WeightedOption {
                            value: 'S',
                            weight: 1,
                        },
                        WeightedOption {
                            value: 'W',
                            weight: 1,
                        },
                    ],
                })
            } else {
                LayoutGeneratorBalance::Balanced(LayoutGeneratorBalancedWeights {
                    envelope_count: vec![
                        WeightedOption {
                            value: 5,
                            weight: 1,
                        },
                        WeightedOption {
                            value: 6,
                            weight: 6,
                        },
                        WeightedOption {
                            value: 7,
                            weight: 1,
                        },
                    ],
                    edge_clock3_count: vec![
                        WeightedOption {
                            value: 2,
                            weight: 4,
                        },
                        WeightedOption {
                            value: 3,
                            weight: 11,
                        },
                        WeightedOption {
                            value: 4,
                            weight: 1,
                        },
                    ],
                    edge_clock6_count: vec![
                        WeightedOption {
                            value: 2,
                            weight: 5,
                        },
                        WeightedOption {
                            value: 3,
                            weight: 3,
                        },
                    ],
                })
            },
            dist_step: 0.2,
        },
    )
    .generate_layout();

    let svg = psi_svg::from_layout(
        &layout,
        700.0,
        std::f64::consts::FRAC_PI_3,
        &psi_svg::PsiSvgAlgo::ForceOnly(
            std::env::var("PLG_CONFIG")
                .ok()
                .map(fs::read_to_string)
                .transpose()?
                .map(|data| json5::from_str(&data))
                .transpose()?
                .unwrap_or_default(),
        ),
    )?;
    print!("{}", svg);
    Ok(())
}
