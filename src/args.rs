use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub docker_support: bool,
}

pub fn args_parser() -> Args {
    Args::parse()
}