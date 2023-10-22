use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(long)]
    pub docker_support: bool,
}
