mod function;
mod promethee;

use crate::promethee::*;
use clap::Clap;

#[derive(Clap, Debug)]
enum PrometheeImplementation {
    #[clap(alias = "van")]
    Vanilla,
    #[clap(alias = "ff")]
    Fast,
}

#[derive(Clap, Debug)]
enum PreferenceFunction {
    #[clap()]
    Usual(UsualFunction),
    #[clap()]
    Quasi(QuasiFunction),
    #[clap()]
    Linear(LinearFunction),
    #[clap()]
    Level(LevelFunction),
    #[clap()]
    LinearWithIndeference(LinearWithIndeferenceFunction),
    #[clap()]
    Gaussian(GaussianFunction),
}

#[derive(Clap, Debug)]
struct Opts {
    #[clap(long, short = 'w', about = "Weight of criteria", required = true)]
    weight: f64,
    #[clap(
        long,
        arg_enum,
        about = "Implementation of Promethee to use",
        required = true
    )]
    version: PrometheeImplementation,
    #[clap(subcommand)]
    function: PreferenceFunction,
}

fn main() {
    let args = Opts::parse();
    // println!("{:#?}", args);

    let criteria = Criteria {
        actions: (0..81).step_by(10).map(|x| x as f64),
        weight: args.weight,
        function: args.function,
        goal: Goal::Min,
    };

    let flow = match args.version {
        PrometheeImplementation::Vanilla => {
            promethee::vanilla::Vanilla::new(true).rank(vec![criteria])
        }
        PrometheeImplementation::Fast => {
            unimplemented!("Fast 'n Furious version wasn't implemented yet.");
        }
    };

    println!("{:#?}", flow);
}
