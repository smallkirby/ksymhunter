use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  pub symbol: String,
  #[clap(long, help = "verbose output")]
  pub verbose: bool,
}
