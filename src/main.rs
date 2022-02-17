/*
 * ksymhunter-rs
 *
 * <---- Original Author's Field ------>
 *
 * Jon Oberheide <jon@oberheide.org>
 * http://jon.oberheide.org
 *
 * Routines for hunting down kernel symbols from from kallsyms,
 * System.map, vmlinux, vmlinuz, and remote symbol servers.
 *
 * System.map parsing adapted from spender's enlightenment.
 *
 *
 * <----------------------------------->
 *
 * <---- smallkirby's field ----------->
 *
 *  Original C program is public at https://github.com/jonoberheide/ksymhunter.
 *
 *  Transported to Rust by smallkirby.
 *
 *  There is no LICENSE statement in original one.
 *
 * <----------------------------------->
 */

use ksymhunter_rs::context::Args;
use ksymhunter_rs::source::Resolver;
use ksymhunter_rs::sysmap::SysmapSource;

use clap::Parser;

fn main() {
  let args = Args::parse();

  let sources: Vec<Box<dyn Resolver>> =
    vec![Box::new(SysmapSource::new("/proc/kallsyms").unwrap())];

  for source in sources {
    let result = match source.resolve(&args.symbol) {
      Ok(result) => result,
      Err(err) => {
        eprintln!("{:?}", err);
        std::process::exit(1);
      }
    };
    if let Some(address) = result {
      println!("{:#X}", address);
      std::process::exit(0);
    }
  }

  std::process::exit(1);
}
