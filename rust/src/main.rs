use clap::Clap;

#[derive(Clap, Debug)]
enum PrometheeImplementation {
    #[clap(alias = "van")]
    Vanilla,
    #[clap(alias = "ff")]
    Fast,
}

#[derive(Clap, Debug)]
struct Opts {
    #[clap(long, arg_enum)]
    version: PrometheeImplementation,
}

fn main() {
    let args = Opts::parse();
    
    match args.version {
        PrometheeImplementation::Vanilla => {
            println!("vanilla");
        },
        PrometheeImplementation::Fast => {
            unimplemented!("Fast 'n Furious version wasn't implemented yet.");
        }
    };
}
