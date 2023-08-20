use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    weight: Option<i32>,

    #[clap(subcommand)]
    unit: Option<Units>,
}

#[derive(Subcommand, Debug)]
enum Units {
    Metric,
    Imperial,
}

const METRIC_MULTIPLIER: f64 = 0.033;
const IMPERIAL_MULTIPLIER: f64 = 0.5;

impl Units {
    fn calc_daily_intake(&self, weight: f64) -> f64 {
        match *self {
            Units::Imperial => f64::trunc(weight * IMPERIAL_MULTIPLIER * 100.0) / 100.0,
            Units::Metric => f64::trunc(weight * METRIC_MULTIPLIER * 100.0) / 100.0,
        }
    }

    fn vol_quantifier(&self) -> &str {
        match *self {
            Units::Imperial => "Ounce(s)",
            Units::Metric => "Litre(s)",
        }
    }
}

fn main() {
    let args = Args::parse();
    let weight = args.weight.expect("could not read weight") as f64;
    let unit = args.unit.unwrap_or(Units::Metric); // emulating default subcommand to metric

    println!(
        "Your daily intake is {:.2} {} of water",
        Units::calc_daily_intake(&unit, weight),
        Units::vol_quantifier(&unit)
    )
}
