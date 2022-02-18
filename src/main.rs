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
use ksymhunter_rs::vmlinux::VmlinuxSource;

use clap::Parser;

fn main() {
  let args = Args::parse();

  let sysmap_sources: Vec<Box<dyn Resolver>> = SysmapSource::from_array(vec![
    "/proc/kallsyms",
    "/proc/ksyms",
    "/boot/System.map-*",
    "/boot/System.map-genkernel-*",
    "/usr/src/linux-%s/System.map",
    "/lib/modules/%s/System.map",
    "/boot/System.map",
    "/System.map",
    "/usr/src/linux/System.map",
  ])
  .into_iter()
  .map(|syssource| Box::new(syssource) as Box<dyn Resolver>)
  .collect();
  let vmlinux_sources: Vec<Box<dyn Resolver>> = VmlinuxSource::from_array(vec![
    "/boot/vmlinux-*",
    "/boot/vmlinux-*.debug",
    "/boot/.debug/vmlinux-*",
    "/boot/.debug/vmlinux-*.debug",
    "/lib/modules/*/vmlinux",
    "/lib/modules/*/vmlinux.debug",
    "/lib/modules/*/.debug/vmlinux",
    "/lib/modules/*/.debug/vmlinux.debug",
    "/usr/lib/debug/lib/modules/*/vmlinux",
    "/usr/lib/debug/lib/modules/*/vmlinux.debug",
    "/usr/lib/debug/boot/vmlinux-*",
    "/usr/lib/debug/boot/vmlinux-*.debug",
    "/usr/lib/debug/vmlinux-*",
    "/usr/lib/debug/vmlinux-*.debug",
    "/var/cache/abrt-di/usr/lib/debug/lib/modules/*/vmlinux",
    "/var/cache/abrt-di/usr/lib/debug/lib/modules/*/vmlinux.debug",
    "/var/cache/abrt-di/usr/lib/debug/boot/vmlinux-*",
    "/var/cache/abrt-di/usr/lib/debug/boot/vmlinux-*.debug",
    "/usr/src/linux-*/vmlinux",
    "/usr/src/linux/vmlinux",
    "/boot/vmlinux",
  ])
  .into_iter()
  .map(|vmlxsource| Box::new(vmlxsource) as Box<dyn Resolver>)
  .collect();

  let sources: Vec<Box<dyn Resolver>> = vec![sysmap_sources, vmlinux_sources]
    .into_iter()
    .flatten()
    .collect();

  for source in sources {
    let result = match source.resolve(&args.symbol) {
      Ok(result) => result,
      Err(err) => {
        // just ignore error and continue to next source
        if args.verbose {
          eprintln!("{:?}", err);
        }
        continue;
      }
    };
    if let Some(address) = result {
      println!("{:#X}", address);
      std::process::exit(0);
    }
  }

  std::process::exit(1);
}
