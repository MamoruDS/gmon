use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(long, short = 'c')]
    pub container_support: bool,
}
